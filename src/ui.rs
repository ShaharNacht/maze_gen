use std::collections::BTreeMap;

use crate::point::WindowPoint;
use crate::{ GFX_UI_X, GFX_UI_Y, GFX_UI_WIDTH, GFX_UI_HEIGHT, GFX_UI_PADDING };

pub struct Ui
{
	mouse: WindowPoint,
	mouse_pressed: bool,
	mouse_pressed_prev: bool,
	
	button_count: i64,
	buttons: BTreeMap< ButtonType, Button >,
	pressed_button: Option<ButtonType>
}

impl Ui
{
	pub fn new() -> Self
	{
		let mouse = WindowPoint::new( 0, 0 );
		let mouse_pressed = false;
		let mouse_pressed_prev = false;
		
		let button_count = ButtonType::all().len() as i64;
		let button_width = ( GFX_UI_WIDTH - GFX_UI_PADDING - button_count * GFX_UI_PADDING ) / button_count;
		let button_height = GFX_UI_HEIGHT - GFX_UI_PADDING * 2;
		
		let mut buttons = BTreeMap::new();
		
		for ( i, button_type ) in ButtonType::all().iter().copied().enumerate()
		{
			let x = GFX_UI_X + GFX_UI_PADDING + ( ( button_width + GFX_UI_PADDING ) * i as i64 );
			let y = GFX_UI_Y + GFX_UI_PADDING;
			
			let button = Button::new( button_type, WindowPoint::new( x, y ), button_width, button_height );
			
			buttons.insert( button_type, button );
		}
		
		Self { mouse, mouse_pressed, mouse_pressed_prev, button_count, buttons, pressed_button: None }
	}
	
	pub fn update( &mut self, mouse: WindowPoint, mouse_pressed: bool )
	{
		self.mouse = mouse;
		self.mouse_pressed = mouse_pressed;
		
		if !self.mouse_pressed
		{
			if let Some(pressed_button) = self.pressed_button
			{
				let button = self.buttons.get_mut(&pressed_button).unwrap();
				button.is_pressed = false;
				button.highlight = 0.0;
				
				self.pressed_button = None;
			}
		}
		
		for button in self.buttons.values_mut()
		{
			let mouse_over = button.is_point_inside(self.mouse);
			
			if mouse_over
			{
				if self.pressed_button.is_none()
				{
					if self.mouse_pressed && !self.mouse_pressed_prev
					{
						button.is_pressed = true;
						self.pressed_button = Some(button.button_type);
					}
					else
					{
						button.highlight += 0.1;
					}
				}
				else
				{
					button.highlight -= 0.1;
				}
			}
			else
			{
				button.highlight -= 0.1;
			}
			
			button.highlight = button.highlight.clamp( 0.0, 1.0 );
		}
		
		self.mouse_pressed_prev = self.mouse_pressed;
	}
	
	pub fn buttons(&self) -> impl Iterator< Item = &Button >
	{
		self.buttons.values()
	}
}

#[derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord )]
pub enum ButtonType
{
	Reset,
	Step,
	PlayOrPause,
	Finish
}

impl ButtonType
{
	fn all() -> &'static [ButtonType]
	{
		&[ ButtonType::Reset, ButtonType::Step, ButtonType::PlayOrPause, ButtonType::Finish ]
	}
}

pub struct Button
{
	pub button_type: ButtonType,
	
	pub position: WindowPoint,
	pub width: i64,
	pub height: i64,
	
	pub highlight: f64,
	pub is_pressed: bool
}

impl Button
{
	fn new( button_type: ButtonType, position: WindowPoint, width: i64, height: i64 ) -> Self
	{
		Self { button_type, position, width, height, highlight: 0.0, is_pressed: false }
	}
	
	fn is_point_inside( &self, point: WindowPoint ) -> bool
	{
		point.x >= self.position.x && point.x < ( self.position.x + self.width ) &&
			point.y >= self.position.y && point.y < ( self.position.y + self.height )
	}
}