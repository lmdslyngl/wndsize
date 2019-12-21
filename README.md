
# wndsize
ウィンドウ名を指定してウィンドウの大きさを変更するコマンドラインツール

## 使い方
例: ウィンドウのタイトルにfirefoxを含むウィンドウの大きさを1280x840に変更する

```
$ wndsize \
    --resize-window \
    --title "$(wndsize --list-windows | grep -i firefox)" \
    --width 1280 \
    --height 840
```

## オプション
- --resize-window
    - ウィンドウの大きさを変更する
- --title
    - 大きさを変更するウィンドウのタイトル
- --width
    - 変更後のウィンドウの幅
- --height
    - 変更後のウィンドウの高さ
- --list-windows
    - ウィンドウのタイトル一覧を表示する
