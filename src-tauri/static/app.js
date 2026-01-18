(function() {
    'use strict';

    // State
    let currentPin = '';
    let config = null;
    let repeatIntervals = {}; // ボタンID -> intervalId のマップ
    let repeatTimeouts = {}; // 長押し開始のタイムアウト
    let ws = null; // WebSocket接続
    let wsReconnectTimer = null;

    // 色の明るさを調整するヘルパー関数
    function adjustBrightness(hex, percent) {
        const num = parseInt(hex.replace('#', ''), 16);
        const r = Math.min(255, Math.max(0, (num >> 16) + percent));
        const g = Math.min(255, Math.max(0, ((num >> 8) & 0x00FF) + percent));
        const b = Math.min(255, Math.max(0, (num & 0x0000FF) + percent));
        return '#' + (0x1000000 + (r << 16) + (g << 8) + b).toString(16).slice(1);
    }

    // 色が暗いかどうかを判定
    function isColorDark(hex) {
        const num = parseInt(hex.replace('#', ''), 16);
        const r = num >> 16;
        const g = (num >> 8) & 0x00FF;
        const b = num & 0x0000FF;
        // 輝度計算
        const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
        return luminance < 0.6;
    }

    // ボタンのスタイルを適用
    function applyButtonStyle(button, color, pressed) {
        const darkerColor = adjustBrightness(color, -30);
        const darkestColor = adjustBrightness(color, -50);
        const lighterColor = adjustBrightness(color, 20);

        if (pressed) {
            button.style.background = `linear-gradient(180deg, ${color} 0%, ${darkerColor} 100%)`;
            button.style.boxShadow = `
                0 1px 0 rgba(255,255,255,0.3) inset,
                0 -1px 0 ${darkerColor} inset,
                0 1px 0 ${darkestColor},
                0 2px 3px rgba(0, 0, 0, 0.2)
            `;
        } else {
            button.style.background = `linear-gradient(180deg, ${lighterColor} 0%, ${color} 100%)`;
            button.style.boxShadow = `
                0 1px 0 ${lighterColor} inset,
                0 -2px 0 ${darkerColor} inset,
                0 4px 0 ${darkestColor},
                0 6px 6px rgba(0, 0, 0, 0.3)
            `;
        }

        // 暗い色のボタンは白文字
        if (isColorDark(color)) {
            button.style.color = 'white';
            button.style.textShadow = '0 1px 2px rgba(0, 0, 0, 0.3)';
        } else {
            button.style.color = '#333';
            button.style.textShadow = 'none';
        }
    }

    // DOM Elements
    const authScreen = document.getElementById('auth-screen');
    const mainScreen = document.getElementById('main-screen');
    const pinInput = document.getElementById('pin-input');
    const authButton = document.getElementById('auth-button');
    const authError = document.getElementById('auth-error');
    const buttonGrid = document.getElementById('button-grid');
    const statusText = document.getElementById('status-text');

    // API calls
    async function apiCall(endpoint, data) {
        try {
            const response = await fetch(endpoint, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(data)
            });
            return await response.json();
        } catch (error) {
            console.error('API Error:', error);
            return { success: false, message: 'Connection error' };
        }
    }

    // 認証
    async function authenticate() {
        const pin = pinInput.value;
        authError.textContent = '';
        authButton.disabled = true;

        const result = await apiCall('/api/auth', { pin });

        if (result.success) {
            currentPin = pin;
            await loadConfig();
            showMainScreen();
        } else {
            authError.textContent = result.message || 'Authentication failed';
            authButton.disabled = false;
        }
    }

    // 設定を読み込み
    async function loadConfig() {
        const result = await apiCall('/api/config', { pin: currentPin });

        if (result.grid) {
            config = result;
            renderButtons();
            statusText.textContent = 'Connected';
        } else {
            statusText.textContent = 'Failed to load config';
        }
    }

    // WebSocket接続
    function connectWebSocket() {
        if (ws && ws.readyState === WebSocket.OPEN) {
            return; // 既にConnected
        }

        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;

        try {
            ws = new WebSocket(wsUrl);

            ws.onopen = () => {
                console.log('WebSocket connected');
                if (wsReconnectTimer) {
                    clearTimeout(wsReconnectTimer);
                    wsReconnectTimer = null;
                }
            };

            ws.onmessage = async (event) => {
                try {
                    const msg = JSON.parse(event.data);
                    if (msg.type === 'ConfigUpdated') {
                        console.log('Config updated, reloading...');
                        await loadConfig();
                    }
                } catch (e) {
                    console.error('WebSocket message parse error:', e);
                }
            };

            ws.onclose = () => {
                console.log('WebSocket disconnected');
                ws = null;
                // 再接続を試みる
                scheduleReconnect();
            };

            ws.onerror = (error) => {
                console.error('WebSocket error:', error);
            };
        } catch (e) {
            console.error('WebSocket connection error:', e);
            scheduleReconnect();
        }
    }

    // 再接続をスケジュール
    function scheduleReconnect() {
        if (wsReconnectTimer) return;
        wsReconnectTimer = setTimeout(() => {
            wsReconnectTimer = null;
            connectWebSocket();
        }, 3000); // 3秒後に再接続
    }

    // ボタンを描画
    function renderButtons() {
        if (!config) return;

        buttonGrid.innerHTML = '';

        // グリッドスタイルを設定
        buttonGrid.style.gridTemplateColumns = `repeat(${config.grid.columns}, 1fr)`;
        buttonGrid.style.gridTemplateRows = `repeat(${config.grid.rows}, 1fr)`;

        // ボタンを作成
        config.buttons.forEach(btn => {
            const button = document.createElement('button');
            button.className = 'grid-button';
            button.textContent = btn.label;
            button.dataset.id = btn.id;

            // グリッド位置を設定
            button.style.gridColumn = `${btn.position.x + 1} / span ${btn.position.width}`;
            button.style.gridRow = `${btn.position.y + 1} / span ${btn.position.height}`;

            // 色を設定（立体感のあるスタイル）
            if (btn.color) {
                const color = btn.color;
                button.dataset.color = color;
                applyButtonStyle(button, color, false);
            }

            // リピート設定を保存
            if (btn.repeat && btn.repeat.enabled) {
                button.dataset.repeatEnabled = 'true';
                button.dataset.repeatInterval = btn.repeat.interval_ms || 100;
            }

            // イベント
            button.addEventListener('touchstart', handleButtonPress, { passive: true });
            button.addEventListener('touchend', handleButtonRelease);
            button.addEventListener('mousedown', handleButtonPress);
            button.addEventListener('mouseup', handleButtonRelease);
            button.addEventListener('mouseleave', handleButtonRelease);

            buttonGrid.appendChild(button);
        });
    }

    // ボタン押下処理
    function handleButtonPress(e) {
        const button = e.currentTarget;
        const buttonId = button.dataset.id;

        button.classList.add('pressed');
        const color = button.dataset.color;
        if (color) {
            applyButtonStyle(button, color, true);
        }

        // リピート機能が有効な場合
        if (button.dataset.repeatEnabled === 'true') {
            const interval = parseInt(button.dataset.repeatInterval) || 100;

            // 即座に一度実行
            executeAction(buttonId, button);

            // 長押し開始までの遅延（300ms後にリピート開始）
            repeatTimeouts[buttonId] = setTimeout(() => {
                // リピート開始
                repeatIntervals[buttonId] = setInterval(() => {
                    executeAction(buttonId, button);
                }, interval);
            }, 300);
        }
    }

    async function handleButtonRelease(e) {
        const button = e.currentTarget;
        const buttonId = button.dataset.id;

        button.classList.remove('pressed');

        // スタイルを元に戻す
        const color = button.dataset.color;
        if (color) {
            applyButtonStyle(button, color, false);
        }

        // リピートのタイムアウトとインターバルをクリア
        if (repeatTimeouts[buttonId]) {
            clearTimeout(repeatTimeouts[buttonId]);
            delete repeatTimeouts[buttonId];
        }
        if (repeatIntervals[buttonId]) {
            clearInterval(repeatIntervals[buttonId]);
            delete repeatIntervals[buttonId];
        }

        if (e.type === 'mouseleave') return;

        // タッチイベントの場合、後続のマウスイベントをキャンセル
        if (e.type === 'touchend') {
            e.preventDefault();
        }

        // リピート機能が有効なボタンは押下時に既に実行済みなのでスキップ
        if (button.dataset.repeatEnabled === 'true') {
            return;
        }

        await executeAction(buttonId, button);
    }

    // アクション実行
    async function executeAction(buttonId, buttonElement) {
        const result = await apiCall('/api/action', {
            button_id: buttonId,
            pin: currentPin
        });

        if (result.success) {
            buttonElement.classList.add('success');
            setTimeout(() => {
                buttonElement.classList.remove('success');
            }, 300);
        } else {
            statusText.textContent = result.message || 'エラー';
            setTimeout(() => {
                statusText.textContent = 'Connected';
            }, 2000);
        }
    }

    // 画面切り替え
    function showMainScreen() {
        authScreen.classList.add('hidden');
        mainScreen.classList.remove('hidden');
        // WebSocket接続を開始
        connectWebSocket();
    }

    // URLパラメータからPINを取得
    function getPinFromUrl() {
        const params = new URLSearchParams(window.location.search);
        return params.get('pin') || '';
    }

    // ローカルストレージからPINを復元
    function restorePin() {
        // URLパラメータを優先
        const urlPin = getPinFromUrl();
        if (urlPin) {
            return urlPin;
        }
        return localStorage.getItem('tapkey_pin') || '';
    }

    // PINを保存
    function savePin(pin) {
        localStorage.setItem('tapkey_pin', pin);
    }

    // 初期化
    function init() {
        const savedPin = restorePin();
        pinInput.value = savedPin;

        // リロードボタン
        const reloadBtn = document.getElementById('reload-btn');
        if (reloadBtn) {
            reloadBtn.addEventListener('click', () => {
                location.reload();
            });
        }

        // 認証ボタン
        authButton.addEventListener('click', async () => {
            await authenticate();
            if (currentPin !== null) {
                savePin(currentPin);
            }
        });

        // Enterキーで認証
        pinInput.addEventListener('keypress', async (e) => {
            if (e.key === 'Enter') {
                await authenticate();
                if (currentPin !== null) {
                    savePin(currentPin);
                }
            }
        });

        // URLパラメータまたは保存済みPINで自動認証を試みる
        setTimeout(async () => {
            const pinToTry = savedPin || '';
            const result = await apiCall('/api/auth', { pin: pinToTry });
            if (result.success) {
                currentPin = pinToTry;
                savePin(currentPin);
                await loadConfig();
                showMainScreen();
            }
        }, 100);
    }

    // DOM読み込み完了時に初期化
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
