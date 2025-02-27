#![allow(dead_code)]

use bevy::prelude::*;
use easer::functions::*;

pub struct EasingPlugin;

impl Plugin for EasingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TimeEase>()
            .add_systems(Update, update_time_ease);
    }
}

/// The Ease struct is used to get more natural values for movements
#[derive(Reflect)]
pub struct EaseStruct {
    current_step: u16,
    total_steps: u16,
    start_val: f32,
    end_val: f32,
    easing_function: EasingFunction,
    easing_type: EasingType,
}

/// Ticks all [TimeEase] timers
fn update_time_ease(mut time_eases: Query<&mut TimeEase>, time: Res<Time>) {
    time_eases.iter_mut().for_each(|mut ease| {
        if ease.is_done() {
            let end_val = ease.get_end_val();
            ease.set_previous_zoom(end_val);
        } else {
            ease.0.increase_step(time.delta().as_millis() as u16);
        }
    })
}

#[derive(Reflect, Component)]
pub struct TimeEase(EaseStruct);

impl TimeEase {
    /// Returns whether the tweening is done or not.
    pub fn is_done(&self) -> bool {
        self.0.is_done()
    }

    /// Setter for previous zoom value.
    fn set_previous_zoom(&mut self, zoom: f32) {
        self.0.set_start_val(zoom);
    }

    /// Only way you should interact with zoom level.
    #[allow(dead_code)]
    pub fn set_ease(
        &mut self,
        goal_zoom: f32,
        easing_function: EasingFunction,
        easing_type: EasingType,
        ms: u16,
    ) {
        *self = TimeEase(EaseStruct::new(
            0,
            ms,
            self.0.get_current_value(),
            goal_zoom,
            easing_function,
            easing_type,
        ))
    }

    /// Forces a zoom value
    #[allow(dead_code)]
    pub fn force_zoom(&mut self, goal_zoom: f32) {
        self.0.force_done();
        self.0.set_end_val(goal_zoom);
    }

    /// Gets the value the ease is approaching
    pub fn get_end_val(&self) -> f32 {
        self.0.get_end_val()
    }

    /// Gets the current eased value
    pub fn get_current_value(&self) -> f32 {
        self.0.get_current_value()
    }

    /// Skips a step
    pub fn increase_step(&mut self, amount: u16) {
        self.0.increase_step(amount);
    }

    /// Forces the easing function to a specific step
    pub fn set_step(&mut self, step: u16) {
        self.0.set_step(step)
    }

    /// Creates a new TimeEase
    pub fn new(
        current_step: u16,
        total_steps: u16,
        start_val: f32,
        end_val: f32,
        easing_function: EasingFunction,
        easing_type: EasingType,
    ) -> Self {
        TimeEase(EaseStruct::new(
            current_step,
            total_steps,
            start_val,
            end_val,
            easing_function,
            easing_type,
        ))
    }

    /// Forcefully skips the easing process
    pub fn force_done(&mut self) {
        self.0.force_done();
    }

    /// Changes the final value
    pub fn set_end_val(&mut self, end_val: f32) {
        self.0.set_end_val(end_val);
    }
}

impl Default for TimeEase {
    fn default() -> Self {
        TimeEase(EaseStruct::new(
            1,
            1,
            1.,
            1.,
            EasingFunction::Sine,
            EasingType::InOut,
        ))
    }
}

impl EaseStruct {
    /// Returns a value between 0. and 1., going off of an EaseStruct.
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
            EasingFunction::None => ease_none(self),
        }
    }

    /// Returns the linear progress that has been made.
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

    pub fn set_step(&mut self, step: u16) {
        self.current_step = step;
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
#[derive(Reflect, Default)]
pub enum EasingFunction {
    #[default]
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
    None,
}

/// Easing types enum.
#[derive(Reflect, Default)]
pub enum EasingType {
    In,
    Out,
    #[default]
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

/// Easing function that slightly overshoots its goal, then returns
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

/// None easing function
fn ease_none(ease: &EaseStruct) -> f32 {
    ease.progress_normalized()
}
