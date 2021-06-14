ngs-fast-loader
===

NGSのゲームスタート前のクソ長いファイルチェックをスキップし、ゲームへ素早くログインするためのツールです。</br>
具体的にはユーザーの`Documents\SEGA\PHANTASYSTARONLINE2`にある`_version.ver`というファイルを`version.ver` にコピーするだけですが、消えたり消えなかったりしてめんどくさいのでツールでディレクトリを監視させて消えたらコピーするだけ、という感じです。</br>
どうやらアップデートのための追加ファイル等を逐一チェックしているようなので、これをずっと使い続けるのはよくないです。</br>
再起動するだけで、すぐにゲーム戻るのにチェックいらんべ！とかいう時にお使いください。</br>
後なんか問題起きても私は責任を負いかねます。自己責任で。</br>

## Usage

`ngs-fast-loader.exe`を先に起動した状態で、NGSのランチャーを起動してください。</br>
うまく起動してたら黒い背景のよくわからんウィンドウが出てきて、そこに`ngs fast loader started`みたいな文字が出てきます。</br>
あとは`version.ver`が消えたら勝手にコピーしてくれます。</br>
止めたかったら黒い背景のよくわからんウィンドウを閉じてください。</br>

## Download

こちらから: [Release](https://github.com/2vg/ngs-fast-loader/releases)</br>
`Assets`とかいうとこの`ngs-fast-loader.exe`をクリックしてダウンロード。</br>

## Build

rust入れといてね

```shell
git clone https://github.com/2vg/ngs-fast-loader
cd ngs-fast-loader
cargo build --release
```
