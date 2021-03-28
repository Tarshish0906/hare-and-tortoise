// ユークリッドの互除法
use nannou::prelude::*;

// アプリケーションの状態を定義
struct Model {}

// アプリケーションの開始
fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    // イベントハンドラなどを設定
    app.new_window()
        .size(500, 500)
        .view(view)
        .build()
        .unwrap();

// イベントハンドラ
fn event(_app: &App, _model: &mut Model, _event: Event) {
}

// 画面の描画処理
fn view(_app: &App, _model: &Model, _frame: Frame) {
    // 縦横比が num_a : num_b の長方形を正方形によって分割
    let mut num_a = 10;
    let mut num_b = 6;
    // 長方形の拡大倍率
    let scalar = 50;
    // 数値の大きさを拡大
    num_a *= scalar;
    num_b *= scalar;

    // プログラム中に動く関数

    // 分割に使う正方形の幅の大きさ（初期値 num_b）
    let wd = num_b;
    // 正方形の x 位置（初期値 0）
    let mut x_pos = 0;
    // 正方形の y 位置（初期値 0）
    let mut y_pos = 0;
    // 分割の繰り返し関数（初期値 0）
    let itr = 0;

    // 描画

    // 繰り返し処理
    // 幅が0になるまで以下を実行
    while(wd > 0){
        // 繰り返し回数を1増やす
        itr += 1;


    }
}