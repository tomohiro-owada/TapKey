<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { AppConfig, ButtonConfig, ShortcutAction, TextAndEnterAction } from './types';

const activeTab = ref<'server' | 'buttons'>('server');
const config = ref<AppConfig | null>(null);
const serverUrl = ref('');
const qrCode = ref('');
const showQrModal = ref(false);
const editingButton = ref<ButtonConfig | null>(null);
const showButtonModal = ref(false);
const isSaving = ref(false);
const saveMessage = ref('');
const draggingButton = ref<ButtonConfig | null>(null);
const dragOffset = ref({ x: 0, y: 0 });
const dragPosition = ref({ x: 0, y: 0 });

// 設定を読み込み
async function loadConfig() {
  try {
    config.value = await invoke<AppConfig>('get_config');
    serverUrl.value = await invoke<string>('get_server_url');
  } catch (e) {
    console.error('設定読み込み失敗:', e);
  }
}

// 設定を保存
async function saveConfig() {
  if (!config.value) return;

  isSaving.value = true;
  saveMessage.value = '';

  try {
    await invoke('save_config', { config: config.value });
    // 保存後に設定を再読み込みしてUIを更新
    await loadConfig();
    saveMessage.value = '保存しました';
    setTimeout(() => saveMessage.value = '', 2000);
  } catch (e) {
    saveMessage.value = '保存に失敗しました';
    console.error('設定保存失敗:', e);
  } finally {
    isSaving.value = false;
  }
}

// QRコードを生成
async function generateQrCode() {
  try {
    qrCode.value = await invoke<string>('get_qr_code');
    showQrModal.value = true;
  } catch (e) {
    console.error('QRコード生成失敗:', e);
  }
}

// ボタン編集モーダルを開く
function openButtonEditor(button?: ButtonConfig) {
  if (button) {
    editingButton.value = JSON.parse(JSON.stringify(button));
  } else {
    editingButton.value = {
      id: `btn_${Date.now()}`,
      label: '新規ボタン',
      position: { x: 0, y: 0, width: 1, height: 1 },
      action: { type: 'shortcut', keys: [] },
      color: '#3498db'
    };
  }
  showButtonModal.value = true;
}

// ボタンを保存
function saveButton() {
  if (!config.value || !editingButton.value) return;

  const index = config.value.buttons.findIndex(b => b.id === editingButton.value!.id);
  if (index >= 0) {
    config.value.buttons[index] = editingButton.value;
  } else {
    config.value.buttons.push(editingButton.value);
  }

  showButtonModal.value = false;
  editingButton.value = null;
}

// ボタンを削除
function deleteButton(id: string) {
  if (!config.value) return;
  config.value.buttons = config.value.buttons.filter(b => b.id !== id);
}

// アクションタイプを切り替え
function changeActionType(type: 'shortcut' | 'text_and_enter') {
  if (!editingButton.value) return;

  if (type === 'shortcut') {
    editingButton.value.action = { type: 'shortcut', keys: [] };
  } else {
    editingButton.value.action = { type: 'text_and_enter', text: '' };
  }
}

// キーを追加
function addKey() {
  if (!editingButton.value || editingButton.value.action.type !== 'shortcut') return;
  (editingButton.value.action as ShortcutAction).keys.push('');
}

// キーを削除
function removeKey(index: number) {
  if (!editingButton.value || editingButton.value.action.type !== 'shortcut') return;
  (editingButton.value.action as ShortcutAction).keys.splice(index, 1);
}

// ボタンのアクション説明を取得
function getButtonActionText(button: ButtonConfig): string {
  if (button.action.type === 'shortcut') {
    return (button.action as ShortcutAction).keys.join(' + ');
  }
  return 'テキスト入力 + Enter';
}

// 編集中のショートカットキー
const editingKeys = computed({
  get: () => {
    if (!editingButton.value || editingButton.value.action.type !== 'shortcut') return [];
    return (editingButton.value.action as ShortcutAction).keys;
  },
  set: (value: string[]) => {
    if (!editingButton.value || editingButton.value.action.type !== 'shortcut') return;
    (editingButton.value.action as ShortcutAction).keys = value;
  }
});

// 編集中のテキスト
const editingText = computed({
  get: () => {
    if (!editingButton.value || editingButton.value.action.type !== 'text_and_enter') return '';
    return (editingButton.value.action as TextAndEnterAction).text;
  },
  set: (value: string) => {
    if (!editingButton.value || editingButton.value.action.type !== 'text_and_enter') return;
    (editingButton.value.action as TextAndEnterAction).text = value;
  }
});

// キーを更新
function updateKey(index: number, value: string) {
  if (!editingButton.value || editingButton.value.action.type !== 'shortcut') return;
  (editingButton.value.action as ShortcutAction).keys[index] = value;
}

// リピートのオン/オフを切り替え
function toggleRepeat(enabled: boolean) {
  if (!editingButton.value) return;
  if (enabled) {
    editingButton.value.repeat = { enabled: true, interval_ms: 100 };
  } else {
    editingButton.value.repeat = undefined;
  }
}

// リピート間隔を更新
function updateRepeatInterval(intervalMs: number) {
  if (!editingButton.value || !editingButton.value.repeat) return;
  editingButton.value.repeat.interval_ms = intervalMs;
}

// 修飾キーの選択肢
const modifierKeys = ['Meta', 'Control', 'Alt', 'Shift'];
const commonKeys = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'Return', 'Tab', 'Space', 'Escape', 'Backspace', 'Delete', 'Up', 'Down', 'Left', 'Right', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12'];

// マウスベースのドラッグ&ドロップ
function onMouseDown(button: ButtonConfig, e: MouseEvent) {
  e.preventDefault();
  draggingButton.value = button;

  const target = e.currentTarget as HTMLElement;
  const rect = target.getBoundingClientRect();
  dragOffset.value = {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top
  };
  dragPosition.value = { x: e.clientX, y: e.clientY };

  document.addEventListener('mousemove', onMouseMove);
  document.addEventListener('mouseup', onMouseUp);
}

function onMouseMove(e: MouseEvent) {
  if (!draggingButton.value) return;
  dragPosition.value = { x: e.clientX, y: e.clientY };
}

function onMouseUp(e: MouseEvent) {
  if (!draggingButton.value || !config.value) {
    cleanup();
    return;
  }

  // preview-gridを探す
  const grid = document.querySelector('.preview-grid') as HTMLElement;
  if (!grid) {
    cleanup();
    return;
  }

  const rect = grid.getBoundingClientRect();
  const cellWidth = (rect.width - 16) / config.value.grid.columns;
  const cellHeight = (rect.height - 16) / config.value.grid.rows;

  // ドロップ位置からグリッド座標を計算
  const x = Math.floor((e.clientX - rect.left - 8) / cellWidth);
  const y = Math.floor((e.clientY - rect.top - 8) / cellHeight);

  // グリッド範囲内かチェック
  if (x >= 0 && x < config.value.grid.columns && y >= 0 && y < config.value.grid.rows) {
    // 範囲内に収める
    const newX = Math.max(0, Math.min(x, config.value.grid.columns - draggingButton.value.position.width));
    const newY = Math.max(0, Math.min(y, config.value.grid.rows - draggingButton.value.position.height));

    // 位置を更新
    const btn = config.value.buttons.find(b => b.id === draggingButton.value!.id);
    if (btn) {
      btn.position.x = newX;
      btn.position.y = newY;
    }
  }

  cleanup();
}

function cleanup() {
  draggingButton.value = null;
  document.removeEventListener('mousemove', onMouseMove);
  document.removeEventListener('mouseup', onMouseUp);
}

// グリッドセルの配列を生成
const gridCells = computed(() => {
  if (!config.value) return [];
  const cells = [];
  for (let y = 0; y < config.value.grid.rows; y++) {
    for (let x = 0; x < config.value.grid.columns; x++) {
      cells.push({ x, y });
    }
  }
  return cells;
});

onMounted(async () => {
  await loadConfig();

  // QRコード表示イベントをリッスン
  await listen('show-qr', () => {
    generateQrCode();
  });
});
</script>

<template>
  <div class="app">
    <header class="header">
      <h1>TapKey 設定</h1>
    </header>

    <nav class="tabs">
      <button
        :class="['tab', { active: activeTab === 'server' }]"
        @click="activeTab = 'server'"
      >
        サーバー設定
      </button>
      <button
        :class="['tab', { active: activeTab === 'buttons' }]"
        @click="activeTab = 'buttons'"
      >
        ボタン設定
      </button>
    </nav>

    <main class="content" v-if="config">
      <!-- サーバー設定タブ -->
      <section v-show="activeTab === 'server'" class="tab-content">
        <div class="form-group">
          <label>ポート番号</label>
          <input type="number" v-model.number="config.port" min="1024" max="65535" />
        </div>

        <div class="form-group">
          <label>PIN（空の場合は認証なし）</label>
          <input type="text" v-model="config.pin" placeholder="4桁の数字など" />
        </div>

        <div class="form-group">
          <label>サーバーURL</label>
          <div class="url-display">
            <code>{{ serverUrl }}</code>
            <button class="btn btn-secondary" @click="generateQrCode">QRコード</button>
          </div>
        </div>

        <div class="info-box">
          <p>スマホのブラウザでこのURLにアクセスするか、QRコードをスキャンしてください。</p>
          <p>※ 同じWi-Fiネットワークに接続している必要があります。</p>
        </div>
      </section>

      <!-- ボタン設定タブ -->
      <section v-show="activeTab === 'buttons'" class="tab-content">
        <div class="form-row">
          <div class="form-group">
            <label>グリッド列数</label>
            <input type="number" v-model.number="config.grid.columns" min="1" max="12" />
          </div>
          <div class="form-group">
            <label>グリッド行数</label>
            <input type="number" v-model.number="config.grid.rows" min="1" max="6" />
          </div>
        </div>

        <!-- グリッドプレビュー -->
        <div class="grid-preview">
          <h3>プレビュー（ドラッグで移動可能、ダブルクリックで編集）</h3>
          <div
            class="preview-grid"
            :class="{ 'is-dragging': draggingButton }"
            :style="{
              gridTemplateColumns: `repeat(${config.grid.columns}, 1fr)`,
              gridTemplateRows: `repeat(${config.grid.rows}, 60px)`
            }"
          >
            <!-- グリッドセル -->
            <div
              v-for="cell in gridCells"
              :key="`cell-${cell.x}-${cell.y}`"
              class="grid-cell"
              :class="{ 'drag-over': draggingButton }"
              :style="{
                gridColumn: cell.x + 1,
                gridRow: cell.y + 1
              }"
            >
              <span class="cell-coord">{{ cell.x }},{{ cell.y }}</span>
            </div>
            <!-- ボタン -->
            <div
              v-for="button in config.buttons"
              :key="button.id"
              class="preview-button"
              :class="{ dragging: draggingButton?.id === button.id }"
              :style="{
                gridColumn: `${button.position.x + 1} / span ${button.position.width}`,
                gridRow: `${button.position.y + 1} / span ${button.position.height}`,
                backgroundColor: button.color || '#3498db'
              }"
              @mousedown="onMouseDown(button, $event)"
              @dblclick="openButtonEditor(button)"
            >
              {{ button.label }}
            </div>
          </div>
        </div>

        <div class="button-list">
          <h3>登録ボタン</h3>
          <div
            v-for="button in config.buttons"
            :key="button.id"
            class="button-item"
            :style="{ borderLeftColor: button.color || '#3498db' }"
          >
            <div class="button-info">
              <strong>{{ button.label }}</strong>
              <span class="button-detail">
                {{ getButtonActionText(button) }}
              </span>
            </div>
            <div class="button-actions">
              <button class="btn btn-small" @click="openButtonEditor(button)">編集</button>
              <button class="btn btn-small btn-danger" @click="deleteButton(button.id)">削除</button>
            </div>
          </div>
        </div>

        <button class="btn btn-primary" @click="openButtonEditor()">+ ボタン追加</button>
      </section>
    </main>

    <!-- ドラッグ中のゴースト表示 -->
    <div
      v-if="draggingButton"
      class="drag-ghost"
      :style="{
        left: dragPosition.x + 'px',
        top: dragPosition.y + 'px',
        backgroundColor: draggingButton.color || '#3498db'
      }"
    >
      {{ draggingButton.label }}
    </div>

    <footer class="footer">
      <span class="save-message" :class="{ visible: saveMessage }">{{ saveMessage }}</span>
      <button class="btn btn-primary" @click="saveConfig" :disabled="isSaving">
        {{ isSaving ? '保存中...' : '設定を保存' }}
      </button>
    </footer>

    <!-- QRコードモーダル -->
    <div v-if="showQrModal" class="modal" @click.self="showQrModal = false">
      <div class="modal-content">
        <h2>QRコード</h2>
        <img v-if="qrCode" :src="qrCode" alt="QR Code" class="qr-image" />
        <p class="qr-url">{{ serverUrl }}</p>
        <button class="btn btn-secondary" @click="showQrModal = false">閉じる</button>
      </div>
    </div>

    <!-- ボタン編集モーダル -->
    <div v-if="showButtonModal && editingButton" class="modal" @click.self="showButtonModal = false">
      <div class="modal-content modal-large">
        <h2>{{ editingButton.id.startsWith('btn_') ? 'ボタン追加' : 'ボタン編集' }}</h2>

        <div class="form-group">
          <label>ラベル</label>
          <input type="text" v-model="editingButton.label" />
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>X位置</label>
            <input type="number" v-model.number="editingButton.position.x" min="0" />
          </div>
          <div class="form-group">
            <label>Y位置</label>
            <input type="number" v-model.number="editingButton.position.y" min="0" />
          </div>
          <div class="form-group">
            <label>幅</label>
            <input type="number" v-model.number="editingButton.position.width" min="1" />
          </div>
          <div class="form-group">
            <label>高さ</label>
            <input type="number" v-model.number="editingButton.position.height" min="1" />
          </div>
        </div>

        <div class="form-group">
          <label>色</label>
          <input type="color" v-model="editingButton.color" />
        </div>

        <div class="form-group">
          <label>アクションタイプ</label>
          <select :value="editingButton.action.type" @change="changeActionType(($event.target as HTMLSelectElement).value as any)">
            <option value="shortcut">ショートカットキー</option>
            <option value="text_and_enter">テキスト入力 + Enter</option>
          </select>
        </div>

        <!-- ショートカット設定 -->
        <div v-if="editingButton.action.type === 'shortcut'" class="form-group">
          <label>キー設定</label>
          <div class="keys-list">
            <div
              v-for="(key, index) in editingKeys"
              :key="index"
              class="key-item"
            >
              <select :value="key" @change="updateKey(index, ($event.target as HTMLSelectElement).value)">
                <optgroup label="修飾キー">
                  <option v-for="k in modifierKeys" :key="k" :value="k">{{ k }}</option>
                </optgroup>
                <optgroup label="通常キー">
                  <option v-for="k in commonKeys" :key="k" :value="k">{{ k }}</option>
                </optgroup>
              </select>
              <button class="btn btn-small btn-danger" @click="removeKey(index)">×</button>
            </div>
          </div>
          <button class="btn btn-small" @click="addKey">+ キー追加</button>
        </div>

        <!-- テキスト入力設定 -->
        <div v-else class="form-group">
          <label>入力テキスト</label>
          <input type="text" v-model="editingText" />
        </div>

        <!-- リピート設定 -->
        <div class="form-group">
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="editingButton.repeat?.enabled"
              @change="toggleRepeat(($event.target as HTMLInputElement).checked)"
            />
            長押しでキーをリピート送信
          </label>
          <div v-if="editingButton.repeat?.enabled" class="repeat-settings">
            <label>リピート間隔 (ミリ秒)</label>
            <input
              type="number"
              :value="editingButton.repeat?.interval_ms || 100"
              @input="updateRepeatInterval(parseInt(($event.target as HTMLInputElement).value) || 100)"
              min="20"
              max="1000"
            />
          </div>
        </div>

        <div class="modal-actions">
          <button class="btn btn-secondary" @click="showButtonModal = false">キャンセル</button>
          <button class="btn btn-primary" @click="saveButton">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --bg-color: #1a1a2e;
  --card-bg: #16213e;
  --text-color: #eaeaea;
  --primary-color: #e94560;
  --secondary-color: #0f3460;
  --border-color: #333;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Hiragino Sans', 'Noto Sans JP', sans-serif;
  background-color: var(--bg-color);
  color: var(--text-color);
}

.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.header {
  padding: 20px;
  background-color: var(--card-bg);
  border-bottom: 1px solid var(--border-color);
}

.header h1 {
  font-size: 20px;
  font-weight: 600;
}

.tabs {
  display: flex;
  background-color: var(--card-bg);
  border-bottom: 1px solid var(--border-color);
}

.tab {
  flex: 1;
  padding: 12px;
  background: none;
  border: none;
  color: #888;
  font-size: 14px;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s;
}

.tab:hover {
  color: var(--text-color);
}

.tab.active {
  color: var(--primary-color);
  border-bottom-color: var(--primary-color);
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.tab-content {
  max-width: 600px;
  margin: 0 auto;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: #888;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 12px;
  background-color: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-color);
  font-size: 16px;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--primary-color);
}

.form-group input[type="color"] {
  height: 44px;
  padding: 4px;
}

.form-row {
  display: flex;
  gap: 16px;
}

.form-row .form-group {
  flex: 1;
}

.url-display {
  display: flex;
  gap: 12px;
  align-items: center;
}

.url-display code {
  flex: 1;
  padding: 12px;
  background-color: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 14px;
}

.info-box {
  padding: 16px;
  background-color: var(--secondary-color);
  border-radius: 8px;
  font-size: 14px;
  line-height: 1.6;
}

.info-box p + p {
  margin-top: 8px;
}

.button-list {
  margin-bottom: 20px;
}

.button-list h3 {
  font-size: 14px;
  color: #888;
  margin-bottom: 12px;
}

.button-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: var(--card-bg);
  border-radius: 8px;
  border-left: 4px solid;
  margin-bottom: 8px;
}

.button-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.button-detail {
  font-size: 12px;
  color: #888;
}

.button-actions {
  display: flex;
  gap: 8px;
}

.grid-preview {
  margin-top: 24px;
}

.grid-preview h3 {
  font-size: 14px;
  color: #888;
  margin-bottom: 12px;
}

.preview-grid {
  display: grid;
  gap: 4px;
  padding: 8px;
  background-color: var(--card-bg);
  border-radius: 8px;
  position: relative;
}

.grid-cell {
  background-color: rgba(255, 255, 255, 0.05);
  border: 1px dashed rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60px;
}

.grid-cell .cell-coord {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.3);
}

.grid-cell.drag-over {
  border-color: var(--primary-color);
  background-color: rgba(233, 69, 96, 0.1);
}

.preview-button {
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  font-size: 12px;
  font-weight: 600;
  border-radius: 6px;
  cursor: grab;
  transition: opacity 0.2s, transform 0.2s;
  z-index: 10;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.preview-button:hover {
  transform: scale(1.02);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

.preview-button:active {
  cursor: grabbing;
}

.preview-button.dragging {
  opacity: 0.3;
  z-index: 5;
}

.drag-ghost {
  position: fixed;
  padding: 12px 24px;
  color: white;
  font-size: 12px;
  font-weight: 600;
  border-radius: 6px;
  pointer-events: none;
  z-index: 1000;
  transform: translate(-50%, -50%);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
  opacity: 0.9;
}


.btn {
  padding: 12px 24px;
  font-size: 14px;
  font-weight: 600;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background-color: var(--primary-color);
  color: white;
}

.btn-secondary {
  background-color: var(--secondary-color);
  color: white;
}

.btn-danger {
  background-color: #e74c3c;
  color: white;
}

.btn-small {
  padding: 8px 12px;
  font-size: 12px;
}

.footer {
  padding: 16px 20px;
  background-color: var(--card-bg);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 16px;
}

.save-message {
  font-size: 14px;
  color: #2ecc71;
  opacity: 0;
  transition: opacity 0.2s;
}

.save-message.visible {
  opacity: 1;
}

.modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 100;
}

.modal-content {
  background-color: var(--card-bg);
  padding: 24px;
  border-radius: 12px;
  max-width: 400px;
  width: 90%;
  text-align: center;
}

.modal-large {
  max-width: 500px;
  text-align: left;
}

.modal-content h2 {
  margin-bottom: 20px;
}

.qr-image {
  width: 200px;
  height: 200px;
  margin-bottom: 16px;
  border-radius: 8px;
  background-color: white;
}

.qr-url {
  font-family: monospace;
  margin-bottom: 16px;
  padding: 8px;
  background-color: var(--bg-color);
  border-radius: 4px;
}

.keys-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.key-item {
  display: flex;
  gap: 8px;
}

.key-item select {
  flex: 1;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  color: var(--text-color);
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.repeat-settings {
  margin-top: 12px;
  padding: 12px;
  background-color: var(--bg-color);
  border-radius: 8px;
}

.repeat-settings label {
  display: block;
  margin-bottom: 8px;
  font-size: 12px;
  color: #888;
}

.repeat-settings input {
  width: 100%;
  padding: 8px;
  background-color: var(--card-bg);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-color);
  font-size: 14px;
}
</style>
