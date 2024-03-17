use std::collections::BTreeMap;

use crate::maze::Maze;
use crate::point::WindowPoint;
use crate::{ GFX_UI_X, GFX_UI_Y, GFX_UI_WIDTH, GFX_UI_HEIGHT, GFX_UI_PADDING };

pub struct Ui
{
	mouse: WindowPoint,
	mouse_pressed: bool,
	mouse_pressed_prev: bool,
	
	buttons: BTreeMap< ButtonId, Button >,
	pressed_button_id: Option<ButtonId>
}

impl Ui
{
	pub fn new() -> Self
	{
		let mouse = WindowPoint::new( 0, 0 );
		let mouse_pressed = false;
		let mouse_pressed_prev = false;
		
		let button_count = ButtonId::all().len() as i64;
		let button_width = ( GFX_UI_WIDTH - GFX_UI_PADDING - button_count * GFX_UI_PADDING ) / button_count;
		let button_height = GFX_UI_HEIGHT - GFX_UI_PADDING * 2;
		
		let mut buttons = BTreeMap::new();
		
		for ( i, button_id ) in ButtonId::all().iter().copied().enumerate()
		{
			let x = GFX_UI_X + GFX_UI_PADDING + ( ( button_width + GFX_UI_PADDING ) * i as i64 );
			let y = GFX_UI_Y + GFX_UI_PADDING;
			
			let button = Button::new( button_id, WindowPoint::new( x, y ), button_width, button_height );
			
			buttons.insert( button_id, button );
		}
		
		Self { mouse, mouse_pressed, mouse_pressed_prev, buttons, pressed_button_id: None }
	}
	
	pub fn update( &mut self, mouse: WindowPoint, mouse_pressed: bool, maze: &mut Maze )
	{
		self.mouse = mouse;
		self.mouse_pressed = mouse_pressed;
		
		if !self.mouse_pressed
		{
			if let Some(pressed_button_id) = self.pressed_button_id
			{
				let button = self.buttons.get_mut(&pressed_button_id).unwrap();
				
				button.is_pressed = false;
				button.highlight = 0.0;
				
				if button.is_point_inside(self.mouse)
				{
					self.click( pressed_button_id, maze );
				}
				
				self.pressed_button_id = None;
			}
		}
		
		for button in self.buttons.values_mut()
		{
			let mouse_over = button.is_point_inside(self.mouse);
			
			if mouse_over && self.pressed_button_id.is_none()
			{
				if self.mouse_pressed && !self.mouse_pressed_prev
				{
					button.is_pressed = true;
					self.pressed_button_id = Some(button.button_id);
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
			
			button.highlight = button.highlight.clamp( 0.0, 1.0 );
		}
		
		self.mouse_pressed_prev = self.mouse_pressed;
	}
	
	pub fn buttons(&self) -> impl Iterator< Item = &Button >
	{
		self.buttons.values()
	}
	
	fn click( &self, button_id: ButtonId, maze: &mut Maze )
	{
		match button_id
		{
			ButtonId::Step => maze.step(),
			
			_ => {}
		}
	}
}

#[derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord )]
pub enum ButtonId
{
	Step,
	AutoStepOrPause,
	Finish,
	Reset
}

impl ButtonId
{
	fn all() -> &'static [ButtonId]
	{
		&[ ButtonId::Step, ButtonId::AutoStepOrPause, ButtonId::Finish, ButtonId::Reset ]
	}
	
	fn text(&self) -> ( &str, Option<&str> )
	{
		match self
		{
			ButtonId::Step => ( "Step", None ),
			ButtonId::AutoStepOrPause => ( "Auto-Step", Some("Pause") ),
			ButtonId::Finish => ( "Finish", None ),
			ButtonId::Reset => ( "Reset", None )
		}
	}
}

pub struct Button
{
	pub button_id: ButtonId,
	
	pub position: WindowPoint,
	pub width: i64,
	pub height: i64,
	
	pub highlight: f64,
	pub is_pressed: bool
}

impl Button
{
	fn new( button_id: ButtonId, position: WindowPoint, width: i64, height: i64 ) -> Self
	{
		Self { button_id, position, width, height, highlight: 0.0, is_pressed: false }
	}
	
	pub fn text(&self) -> ( &str, Option<&str> )
	{
		self.button_id.text()
	}
	
	fn is_point_inside( &self, point: WindowPoint ) -> bool
	{
		point.x >= self.position.x &&
			point.x < ( self.position.x + self.width ) &&
			point.y >= self.position.y
			&& point.y < ( self.position.y + self.height )
	}
}