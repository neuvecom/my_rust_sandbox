#!/bin/bash

#------------------------#
# 100本ノック用のファイルを
# 生成するスクリプトです。
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
echo -e "-- \033[1;32m100本ノック用のファイル生成ツール\033[0;39m"
echo "-- "
echo "-- Ver.${SCRIPT_VER}"
echo "-- "
echo "-- 以下のディレクトリにファイルを作成します。"
cd ${SCRIPT_DIR}${MAKE_DIR}
echo "-- ${SCRIPT_DIR}${MAKE_DIR}"
echo "-- "
echo "-- ファイル名パターン。"
echo "-- ${FILE_NAME_PREF}xxx${FILE_NAME_END}"
echo "-- "

if [ ! $srartNum ]; then
	echo "-- >>>>"
  echo -e "--  (^_^)ノ \033[1;36m生成をスタートするファイルの番号を入力してください。\033[0;39m"
  IS_WAIT=true
  while $IS_WAIT
  do
    read srartNum
    if [ ${#srartNum} -le 3 ] && [[ "$srartNum" =~ ^[0-9]+$ ]]; then
        IS_WAIT=false
    else
      echo -e "--  (^_^)ノ \033[1;33m3桁以内の整数値を入力してください。\033[0;39m"
    fi
  done
fi
FILE_NUM_START=$(printf "%03d\n" "${srartNum}")

if [ ! $stopNum ]; then
	echo "-- >>>>"
  echo -e "--  (^_^)ノ \033[1;36m生成する最後のファイルの番号を入力してください。\033[0;39m"
  IS_WAIT=true
  while $IS_WAIT
  do
    read stopNum
    if [ ${#stopNum} -le 3 ] && [[ "$stopNum" =~ ^[0-9]+$ ]]; then
      IS_WAIT=false
    else
      echo -e "--  (^_^)ノ \033[1;33m3桁以内の整数値を入力してください。\033[0;39m"
    fi
  done
fi
FILE_NUM_STOP=$(printf "%03d\n" "${stopNum}")

echo "-- "
echo "-- 以下のファイルを生成します。"
echo "-- ${FILE_NAME_PREF}${FILE_NUM_START}${FILE_NAME_END} ～ ${FILE_NAME_PREF}${FILE_NUM_STOP}${FILE_NAME_END}"
echo "-- "

if [ ! $doTaskimg ]; then
	echo -e "-- (^_^)ノ \033[1;31mファイルを生成しますか？\033[0;39m"
	echo "-- "
	echo "-- \033[1;33m [!] 未入力のままenterするとこの処理を回避します。\033[0;39m"
  echo "-- \033[1;36m y や yes など英数文字を入力してください。\033[0;39m"
fi
if [ ! $doTaskimg ]; then
	read doTaskimg
fi

for ((i=$srartNum; i < $stopNum+1; i++)); do
    myFile=${SCRIPT_DIR}${MAKE_DIR}/${FILE_NAME_PREF}$(printf "%03d\n" $i)${FILE_NAME_END}
    touch $myFile
    echo "/*" >> $myFile
    echo "このファイルはスクリプトにより生成されました" >> $myFile
    echo "*/" >> $myFile
    echo "" >> $myFile
    echo "fn main() {" >> $myFile
    echo "" >> $myFile
    echo "}" >> $myFile
done

# 終了処理
echo "  "
echo "----- FINISH ------"
echo "  "
echo "--==--==--==--==--==--==--==--==--==-- "

exit