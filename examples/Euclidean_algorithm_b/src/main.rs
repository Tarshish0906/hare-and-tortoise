// ユークリッドの互除法
use nannou::prelude::*;

    // 横縦比が num_a : num_b の長方形によって分割
    let mut num_a = 10;
    let mut num_b = 6;
    
    // 長方形の拡大倍率
    let scalar = 50;

    // 数値の大きさを拡大
    let num_a *= scalar;
    let num_b *= scalar;

    // プログラム実行中に動く変数
    // 分割に使う正方形の幅の大きさ（初期値 num_b）
    let mut wd = num_b;

    // 正方形のx位置（初期値 0）
    let mut x_pos = 0;
    // 正方形のy位置（初期値 0）
    let mut y_pos = 0;
    // 分割の繰り返し回数（初期値 0）
    let itr = 0;
