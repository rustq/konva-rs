use crate::glue;

pub enum Transaction2DMethod {
    fill_rect,
    fill_stroke,
    set_fill_style,
}

pub struct FillRectTransitionParam {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct SetFillStyleTransitionParam {
    pub color: String,
}

pub struct Context {
    transaction_methods: Vec<Transaction2DMethod>,
    pub fill_rect_parma_list: Vec<FillRectTransitionParam>,
    pub set_fill_style_parma_list: Vec<SetFillStyleTransitionParam>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            transaction_methods: Vec::new(),
            fill_rect_parma_list: Vec::new(),
            set_fill_style_parma_list: Vec::new(),
        }
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.transaction_methods
            .push(Transaction2DMethod::fill_rect);
        self.fill_rect_parma_list.push(FillRectTransitionParam {
            x: x,
            y: y,
            width: width,
            height: height,
        });
    }

    pub fn set_fill_style(&mut self, color: String) {
        self.transaction_methods
            .push(Transaction2DMethod::set_fill_style);
        self.set_fill_style_parma_list
            .push(SetFillStyleTransitionParam { color });
    }

    pub fn get_transaction_methods(&self) -> &Vec<Transaction2DMethod> {
        &self.transaction_methods
    }
}
