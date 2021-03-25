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

// Model のインスタンスを生成
fn model(_app: &App) -> Model {
    Model {}
}

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
}