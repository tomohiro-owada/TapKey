# TapKey

<p align="center">
  <img src="TapKey.png" alt="TapKey Icon" width="128" height="128">
</p>

<p align="center">
  <strong>スマホからMac/PCのキーボードショートカットを発火させるリモコンアプリ</strong>
  <br>
  <strong>Remote keyboard shortcut controller for Mac/PC via smartphone</strong>
</p>

<p align="center">
  <a href="#english">English</a> | <a href="#日本語">日本語</a>
</p>

<p align="center">
  <a href="https://github.com/tomohiro-owada/TapKey/releases/latest">
    <img src="https://img.shields.io/github/v/release/tomohiro-owada/TapKey?style=flat-square" alt="Latest Release">
  </a>
  <a href="https://github.com/tomohiro-owada/TapKey/releases">
    <img src="https://img.shields.io/github/downloads/tomohiro-owada/TapKey/total?style=flat-square" alt="Downloads">
  </a>
  <a href="https://github.com/tomohiro-owada/TapKey/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/tomohiro-owada/TapKey?style=flat-square" alt="License">
  </a>
</p>

---

<h2 id="日本語">日本語</h2>

## 概要

TapKeyは、スマートフォンをMac/Windows PCのリモートキーボードとして使えるアプリです。カスタマイズ可能なボタンを配置して、ショートカットキーやテキスト入力をワンタップで実行できます。

### 主な用途

- **音声入力アプリの操作** - SuperWhisperなどの録音開始/停止
- **プレゼンテーション** - スライド操作のリモコン
- **動画編集** - よく使うショートカットをワンタップで
- **配信** - OBSのシーン切り替えなど

## ダウンロード

### Mac

| チップ | ダウンロード |
|--------|------------|
| Apple Silicon (M1/M2/M3) | [TapKey_aarch64.dmg](https://github.com/tomohiro-owada/TapKey/releases/latest/download/TapKey_aarch64.dmg) |
| Intel | [TapKey_x64.dmg](https://github.com/tomohiro-owada/TapKey/releases/latest/download/TapKey_x64.dmg) |

### Windows

| アーキテクチャ | ダウンロード |
|--------------|------------|
| 64-bit | [TapKey_x64-setup.exe](https://github.com/tomohiro-owada/TapKey/releases/latest/download/TapKey_x64-setup.exe) |

> **Note**: macOSでは初回起動時に「開発元を確認できない」または「壊れている」という警告が出る場合があります。以下のコマンドをターミナルで実行してください：
>
> ```bash
> xattr -cr /Applications/TapKey.app
> ```
>
> または、システム設定 > プライバシーとセキュリティ から「このまま開く」を選択してください。

## 使い方

### 1. アプリをインストール

ダウンロードしたファイルを実行してインストールします。

### 2. アクセシビリティ権限を許可（Mac）

初回起動時にアクセシビリティ権限を求められます。これはキーボード入力をシミュレートするために必要です。

#### 設定方法

1. **システム設定**（macOS Ventura以降）または**システム環境設定**（それ以前）を開く
2. **プライバシーとセキュリティ** > **アクセシビリティ** を選択
3. 左下の鍵アイコンをクリックしてロックを解除
4. リストにある **TapKey** にチェックを入れる

> **Note**: TapKeyがリストにない場合は、「+」ボタンをクリックしてアプリケーションフォルダからTapKeyを追加してください。

#### アクセシビリティ権限が必要な理由

TapKeyは`enigo`ライブラリを使用してキーボード入力をシミュレートします。macOSではセキュリティ上の理由から、他のアプリケーションにキー入力を送信するにはアクセシビリティ権限が必要です。この権限がないとキー送信が機能しません。

#### 権限を許可しても動作しない場合

1. TapKeyを一度終了（メニューバーアイコン → Quit）
2. アクセシビリティのリストからTapKeyのチェックを外す
3. 再度チェックを入れる
4. TapKeyを再起動

### 3. スマホからアクセス

1. Mac/PCとスマホを**同じWi-Fiネットワーク**に接続
2. メニューバー（Mac）またはシステムトレイ（Windows）のTapKeyアイコンをクリック
3. 「QRコード表示」を選択
4. スマホでQRコードをスキャン、またはURLを直接入力

### 4. ボタンをカスタマイズ

設定画面の「ボタン設定」タブで：

- **ボタンを追加** - 「+ ボタン追加」をクリック
- **ラベル** - ボタンに表示するテキスト
- **位置とサイズ** - グリッド上の配置を指定
- **色** - ボタンの背景色
- **アクション** - ショートカットキーまたはテキスト入力
- **長押しリピート** - Backspaceなど連打したいキーに

プレビュー上でドラッグ&ドロップでも位置を変更できます。

### 5. PWAとしてホーム画面に追加（推奨）

スマホのブラウザでTapKeyを開いた状態で：

- **iPhone**: 共有ボタン → 「ホーム画面に追加」
- **Android**: メニュー → 「ホーム画面に追加」

これでアプリのように使えます。

## 機能

### ショートカットキー

修飾キー（Cmd, Ctrl, Alt, Shift）と通常キーの組み合わせを設定できます。

例：
- `Cmd + Shift + R` - SuperWhisperの録音開始
- `Cmd + N` - 新規ファイル
- `F5` - プレゼンテーション開始

### テキスト入力 + Enter

定型文を入力してEnterを送信します。チャットでよく使うフレーズなどに。

### 長押しリピート

Backspaceや矢印キーなど、長押しで連続入力したいキーに設定できます。

### PIN認証

不正アクセスを防ぐためPINを設定できます（任意）。

## セキュリティ

- 通信は同一LANネットワーク内のみ
- PIN認証でアクセス制限可能
- ファイアウォールで必要に応じてポートを制限

## 開発

### 必要な環境

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### セットアップ

```bash
# 依存関係をインストール
npm install

# 開発サーバー起動
npm run tauri dev

# ビルド
npm run tauri build
```

## ライセンス

MIT License

## 作者

[@abalol](https://twitter.com/abalol)

---

<h2 id="english">English</h2>

## Overview

TapKey turns your smartphone into a remote keyboard for your Mac/Windows PC. Configure custom buttons to trigger keyboard shortcuts or text input with a single tap.

### Use Cases

- **Voice input apps** - Start/stop recording with SuperWhisper, etc.
- **Presentations** - Control slides remotely
- **Video editing** - Quick access to frequently used shortcuts
- **Streaming** - OBS scene switching, etc.

## Download

### Mac

| Chip | Download |
|------|----------|
| Apple Silicon (M1/M2/M3) | [TapKey_aarch64.dmg](https://github.com/tomohiro-owada/TapKey/releases/latest/download/TapKey_aarch64.dmg) |
| Intel | [TapKey_x64.dmg](https://github.com/tomohiro-owada/TapKey/releases/latest/download/TapKey_x64.dmg) |

### Windows

| Architecture | Download |
|--------------|----------|
| 64-bit | [TapKey_x64-setup.exe](https://github.com/tomohiro-owada/TapKey/releases/latest/download/TapKey_x64-setup.exe) |

> **Note**: On macOS, you may see a warning saying the app is "damaged" or from an "unidentified developer". Run this command in Terminal to fix it:
>
> ```bash
> xattr -cr /Applications/TapKey.app
> ```
>
> Alternatively, go to System Settings > Privacy & Security and click "Open Anyway".

## Usage

### 1. Install the App

Run the downloaded file to install.

### 2. Grant Accessibility Permission (Mac)

On first launch, you'll be prompted to grant accessibility permission. This is required to simulate keyboard input.

#### How to Configure

1. Open **System Settings** (macOS Ventura+) or **System Preferences** (earlier versions)
2. Go to **Privacy & Security** > **Accessibility**
3. Click the lock icon at bottom-left to unlock
4. Check the box next to **TapKey** in the list

> **Note**: If TapKey isn't in the list, click the "+" button and add TapKey from your Applications folder.

#### Why Accessibility Permission is Required

TapKey uses the `enigo` library to simulate keyboard input. On macOS, sending key events to other applications requires accessibility permission for security reasons. Without this permission, key simulation won't work.

#### If It's Not Working After Granting Permission

1. Quit TapKey (Menu bar icon → Quit)
2. Uncheck TapKey in the Accessibility list
3. Check it again
4. Restart TapKey

### 3. Connect from Your Phone

1. Connect your Mac/PC and phone to the **same Wi-Fi network**
2. Click the TapKey icon in the menu bar (Mac) or system tray (Windows)
3. Select "Show QR Code"
4. Scan the QR code with your phone, or enter the URL directly

### 4. Customize Buttons

In the settings window under "Buttons" tab:

- **Add button** - Click "+ Add Button"
- **Label** - Text displayed on the button
- **Position and size** - Specify grid placement
- **Color** - Button background color
- **Action** - Keyboard shortcut or text input
- **Long-press repeat** - For keys like Backspace that you want to repeat

You can also drag and drop buttons on the preview to reposition them.

### 5. Add to Home Screen as PWA (Recommended)

With TapKey open in your phone's browser:

- **iPhone**: Share button → "Add to Home Screen"
- **Android**: Menu → "Add to Home Screen"

This gives you an app-like experience.

## Features

### Keyboard Shortcuts

Configure combinations of modifier keys (Cmd, Ctrl, Alt, Shift) with regular keys.

Examples:
- `Cmd + Shift + R` - Start SuperWhisper recording
- `Cmd + N` - New file
- `F5` - Start presentation

### Text Input + Enter

Input preset text and send Enter. Great for frequently used chat phrases.

### Long-press Repeat

For keys like Backspace or arrow keys that you want to repeat when held down.

### PIN Authentication

Set a PIN to prevent unauthorized access (optional).

## Security

- Communication is limited to the same LAN
- PIN authentication available for access control
- Configure firewall to restrict port access if needed

## Development

### Requirements

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### Setup

```bash
# Install dependencies
npm install

# Start development server
npm run tauri dev

# Build
npm run tauri build
```

## License

MIT License

## Author

[@abalol](https://twitter.com/abalol)
