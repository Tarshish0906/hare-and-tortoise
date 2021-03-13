// ユークリッドの互除法

use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
    .simple_window(view)
    .size(500, 500)
    .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    // 横縦比が num_a : num_b の長方形によって分割
    let num_a = 10;
    let num_b = 6;
    
    // 長方形の拡大倍率
    let scalar = 50;

    // 数値の大きさを拡大
    let num_a *= scalar;
    let num_b *= scalar;

    // プログラム実行中に動く変数
    // 分割に使う正方形の幅の大きさ（初期値 num_b）
    let wd = num_b;

    // 正方形のx位置（初期値 0）
    let x_pos = 0;
    // 正方形のy位置（初期値 0）
    let y_pos = 0;
    // 分割の繰り返し回数（初期値 0）
    let itr = 0;
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    // 繰り返し処理
    // 幅が0になるまで以下を実行
    while (wd > 0) {
        // 繰り返し回数を1増やす
        itr += ;
    }
}