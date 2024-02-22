#!/bin/bash

#------------------------#
# 100本ノック用のファイルを
# 指定して実行するスクリプトです。
#------------------------#

#------------------------#
# 初期設定
#------------------------#

SCRIPT_VER=20240220-001

SCRIPT_DIR=$(cd $(dirname $(readlink $0 || echo $0));pwd -P)
MAKE_DIR="/src/bin"
FILE_NAME_PREF="knock100_"
FILE_NAME_END=".rs"

#------------------------#
# メイン
#------------------------#

echo "--==--==--==--==--==--==--==--==--==-- "
echo "-- "
echo -e "-- \033[1;32m100本ノック用のファイル実行ツール\033[0;39m"
echo "-- "
echo "-- Ver.${SCRIPT_VER}"
echo "-- "
echo "-- 以下のディレクトリの100本ノックのファイルを実行します。"
cd ${SCRIPT_DIR}${MAKE_DIR}
echo "-- ${SCRIPT_DIR}${MAKE_DIR}"
echo "-- "
echo "-- 環境確認... "

# rustの確認
rust_ver=`rustc --version`
if [ $? -ne 0 ]; then
	echo "-- rustc Error"
	echo "-- [！実行不能] 「rustc」が実行できませんでした。"
	echo "-- [！実行不能] 「rustc/rust」がインストールされていません。"
	echo "-- [！実行不能] curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh を実行してください。"
	exit 1
else
	echo -n "-- Rust OK => "
  echo $rust_ver
fi
# cargoの確認
cargo_ver=`cargo --version`
if [ $? -ne 0 ]; then
	echo "-- Cargo Error"
	echo "-- [！実行不能] 「cargo」が実行できませんでした。"
	echo "-- [！実行不能] 「cargo/rust」がインストールされていません。"
	echo "-- [！実行不能] curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh を実行してください。"
	exit 1
else
	echo -n "-- Cargo OK => "
  echo $cargo_ver
fi

# pecoの確認と実行するファイルの選択
peco_ver=`peco --version`
if [ $? -ne 0 ]; then
  echo "-- peco Error"
  echo "-- [！実行不能] 「peco」が実行できませんでした。"
  echo "-- [！実行不能] 「peco」がインストールされていません。"
  echo "-- [！実行不能] brew install peco を実行してください。"
  exit 1
else
  echo -n "-- peco OK => "
  echo $peco_ver
  echo "-- "
  echo -e "-- (^_^)ノ \033[1;31m実行するファイルを選択してください。\033[0;39m"
  echo "-- "
  echo "-> 実行するファイル名を指定してください"
  echo "-> retuenキーで選択画面に進みます。"
  read Peco
  doFile=`ls -1 | grep "^${FILE_NAME_PREF}" | peco`
fi

# ファイル名の加工
doFile=${doFile/${FILE_NAME_END}/}

# 読込ファイルの指定
if [ "$doFile" = "knock100_054" ] || [ "$doFile" = "knock100_057" ]; then
  echo "-- [！] 読込ファイル指定が必要な課題です。"
  echo -e "-- (^_^)ノ \033[1;31m読込ファイルを選択してください。\033[0;39m"
  echo "-- "
  echo "-> retuenキーで選択画面に進みます。"
  read Peco
  if [ "$doFile" = "knock100_054" ] ; then
    readFile=`ls -1  ./054data | peco`
  fi
  if [ "$doFile" = "knock100_057" ] ; then
    readFile=`ls -1  ./057data | peco`
  fi
  echo "-- 読込ファイル[${readFile}]"
else
  echo "-- [!] 読込ファイル指定が不要な課題です。"
fi

# 実行
echo "-- Cargo Runを実行します。[${doFile}] [${readFile}]"
echo "  "
cargo run --bin $doFile $readFile

# 終了処理
echo "  "
echo "----- FINISH ------"
echo "  "
echo "--==--==--==--==--==--==--==--==--==-- "

exit
