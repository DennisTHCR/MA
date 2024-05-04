#![allow(dead_code)]
// https://easings.net/ and https://github.com/semitable/easing-functions/blob/master/easing_functions/easing.py for reference

use bevy::reflect::Reflect;
use easer::functions::*;

/// The Ease struct is used to get more natural values for e.g. animations.
#[derive(Reflect)]
pub struct EaseStruct {
    current_step: u16,
    total_steps: u16,
    start_val: f32,
    end_val: f32,
    easing_function: EasingFunction,
    easing_type: EasingType,
}

impl EaseStruct {
    /// Returns a value between 0. and 1., going off of an EaseStruct. TODO: Implement easing functions.
    pub fn get_progress_eased(&self) -> f32 {
        match self.easing_function {
            EasingFunction::Sine => ease_sine(self),
            EasingFunction::Quad => ease_quad(self),
            EasingFunction::Cubic => ease_cubic(self),
            EasingFunction::Quart => ease_quart(self),
            EasingFunction::Quint => ease_quint(self),
            EasingFunction::Expo => ease_expo(self),
            EasingFunction::Circ => ease_circ(self),
            EasingFunction::Back => ease_back(self),
            EasingFunction::Elastic => ease_elastic(self),
            EasingFunction::Bounce => ease_bounce(self),
        }
    }

    fn progress_normalized(&self) -> f32 {
        self.current_step as f32 / self.total_steps as f32
    }

    pub fn step(&mut self) {
        if self.current_step == self.total_steps {
            return;
        }
        self.current_step += 1;
    }

    pub fn increase_step(&mut self, amount: u16) {
        if self.current_step == self.total_steps {
            return;
        }
        self.current_step += amount;
        if self.current_step > self.total_steps {
            self.current_step = self.total_steps
        }
    }

    pub fn new(
        current_step: u16,
        total_steps: u16,
        start_val: f32,
        end_val: f32,
        easing_function: EasingFunction,
        easing_type: EasingType,
    ) -> Self {
        EaseStruct {
            current_step,
            total_steps,
            start_val,
            end_val,
            easing_function,
            easing_type,
        }
    }

    pub fn is_done(&self) -> bool {
        self.current_step == self.total_steps
    }

    pub fn force_done(&mut self) {
        self.current_step = self.total_steps;
    }

    pub fn set_start_val(&mut self, start_val: f32) {
        self.start_val = start_val;
    }

    pub fn set_end_val(&mut self, end_val: f32) {
        self.end_val = end_val;
    }

    pub fn get_start_val(&self) -> f32 {
        self.start_val
    }

    pub fn get_end_val(&self) -> f32 {
        self.end_val
    }

    pub fn get_current_value(&self) -> f32 {
        self.start_val + (self.end_val - self.start_val) * self.get_progress_eased()
    }
}

/// Easing functions enum
#[derive(Reflect)]
pub enum EasingFunction {
    Sine,
    Quad,
    Cubic,
    Quart,
    Quint,
    Expo,
    Circ,
    Back,
    Elastic,
    Bounce,
}

/// Easing types enum.
#[derive(Reflect)]
pub enum EasingType {
    In,
    Out,
    InOut,
}

// easing function implementations

/// Sine easing function
fn ease_sine(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Sine::ease_in(t, b, c, d),
        EasingType::InOut => Sine::ease_in_out(t, b, c, d),
        EasingType::Out => Sine::ease_out(t, b, c, d),
    }
}

/// Quadratic easing function
fn ease_quad(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Quad::ease_in(t, b, c, d),
        EasingType::InOut => Quad::ease_in_out(t, b, c, d),
        EasingType::Out => Quad::ease_out(t, b, c, d),
    }
}

/// Cubic easing function
fn ease_cubic(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Cubic::ease_in(t, b, c, d),
        EasingType::InOut => Cubic::ease_in_out(t, b, c, d),
        EasingType::Out => Cubic::ease_out(t, b, c, d),
    }
}

/// Quartic easing function
fn ease_quart(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Quart::ease_in(t, b, c, d),
        EasingType::InOut => Quart::ease_in_out(t, b, c, d),
        EasingType::Out => Quart::ease_out(t, b, c, d),
    }
}

/// Quintic easing function
fn ease_quint(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Quint::ease_in(t, b, c, d),
        EasingType::InOut => Quint::ease_in_out(t, b, c, d),
        EasingType::Out => Quint::ease_out(t, b, c, d),
    }
}

/// Exponential easing function
fn ease_expo(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Expo::ease_in(t, b, c, d),
        EasingType::InOut => Expo::ease_in_out(t, b, c, d),
        EasingType::Out => Expo::ease_out(t, b, c, d),
    }
}

/// Circular easing function
fn ease_circ(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Circ::ease_in(t, b, c, d),
        EasingType::InOut => Circ::ease_in_out(t, b, c, d),
        EasingType::Out => Circ::ease_out(t, b, c, d),
    }
}

/// Easing function that slightly overshoots it's goal, then returns
fn ease_back(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Back::ease_in(t, b, c, d),
        EasingType::InOut => Back::ease_in_out(t, b, c, d),
        EasingType::Out => Back::ease_out(t, b, c, d),
    }
}

/// Elastic easing function
fn ease_elastic(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Elastic::ease_in(t, b, c, d),
        EasingType::InOut => Elastic::ease_in_out(t, b, c, d),
        EasingType::Out => Elastic::ease_out(t, b, c, d),
    }
}

/// Bouncy easing function
fn ease_bounce(ease: &EaseStruct) -> f32 {
    let t = ease.progress_normalized();
    let b = 0.;
    let c = 1.;
    let d = 1.;

    match ease.easing_type {
        EasingType::In => Bounce::ease_in(t, b, c, d),
        EasingType::InOut => Bounce::ease_in_out(t, b, c, d),
        EasingType::Out => Bounce::ease_out(t, b, c, d),
    }
}
