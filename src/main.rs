#![allow(unused)]
#![allow(unused_imports)]
use core::error;
use std::path::PathBuf;
use std::sync::Arc;
use std::{
	error::Error,
	io,
	io::stdin,
	io::Write,
	fs,
	fs::File,
	fs::OpenOptions,
	process,
	env,
	collections::BTreeMap,
};//use serde_ };
use app_data::AppData;
use uuid::Uuid;
//use std::ops::Index;
use csv::WriterBuilder;
use dialoguer::Select;
use dialoguer::Input;
use serde::{Deserialize,Serialize};
//use csv::WriterBuilder;
//use std::fmt::Debug;
//use std::io::*;
//use std::alloc::System;
//use std::process::Command;
//use console::Term;

#[derive(Clone, Debug)]
struct FilamentLibrary {
	pub library_name:String,
	pub id: Uuid,
	pub all_filament:Vec<Filament>,
	recycle_bin:Vec<Filament>,
}

#[derive(Debug)]
struct FilamentLibrarySourceFile {
	source_file:File,
}

#[derive(Clone, Debug,serde::Serialize, serde::Deserialize)]
struct Filament {
	name: String,
	manufacturer: String,
	material: String,
	color: String,
	//spool_count: i8,
}
#[derive(Clone, Debug )]
struct Spool {
	name: Filament,
	weight: i8,
	material: String,
	color: String,
	//spool_count: i8,
}

impl Filament {
	/*fn new() -> Self {
		Self {
			name:String::new(),
			filament_type:String::new(),
			color:String::new(),
			//spool_count::new()
		}
	}*/
}

impl FilamentLibrary {
	fn new( name: String) -> Self {
		Self {
			library_name:name,
			id: Uuid::new_v4(),
			all_filament:Vec::new(),
			recycle_bin:Vec::new(),
		}
	}

	fn create_library_file( &self ){
		let mut app_save_directory = get_app_save_directory();
		let mut app_autosave_directory = get_app_autosave_directory();

		std::fs::create_dir_all(app_save_directory.clone() );
		std::fs::create_dir_all(app_autosave_directory.clone() );
		app_save_directory.push(self.id.clone().to_string() );
		

		let mut sourcefile_full_path = app_save_directory;
		sourcefile_full_path.set_extension("json");
		println!("{}",sourcefile_full_path.display());

		let sourcefile = OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			.open(sourcefile_full_path);

		match sourcefile {
			Ok(mut file) => {
				writeln!(&mut file, "new line");
			},
			Err(error) => {
				println!("Crap!");
				println!("{}",error);
			}
		}
		
   		
	}

	fn open_library_file(){

		//let metadata = file.metadata()?;
	}

	fn export_csv( &self ) -> Result<(), Box<dyn std::error::Error>> {
		println!("Starting Export");

		  let mut wtr = WriterBuilder::new().from_path("Filament Inventory.csv")?;
			wtr.write_record(["Filament Name","Filament Menufacturer", "Filament Material", "Filament Color"])?;
			for ( item ) in self.all_filament.iter() {
				wtr.write_record([item.name.clone(), item.manufacturer.clone(), item.material.clone(), item.color.clone()])?;
			}
			wtr.flush()?;
			Ok(())
	}

	
	fn export_json( &self ){

	}

	fn autosave_filament_library_json( &self ) -> Result<(), Box<dyn std::error::Error>> {
		let autosave_inventory = self.get_inventory_copy();

		//let inventory_library = vec![]

		let autosave_json = serde_json::to_string( &autosave_inventory )?;
		println!("{}", autosave_json);
		Ok(())
	}
	

	fn import_csv( &self ) -> Result<(),Box<dyn Error>> {
		println!("Starting Import");
		let mut imported_filament_inventory = csv::Reader::from_reader(io::stdin());

		for result in imported_filament_inventory.records(){
			let record = result?;
			println!("{:?}", record );
		}
		Ok(())
	}

	fn import_test_filaments( &mut self ){
		self.add_filament( "Overture PLA Pro".to_string(), "Overture".to_string(), "PLA".to_string(), "Black".to_string() );
		self.add_filament(  "Overture PLA Pro".to_string(),"Overture".to_string(), "PLA".to_string(), "Blue".to_string() );
		self.add_filament( "Pokymaker PLA-HT".to_string(),"Polymaker".to_string(), "PLA-HT".to_string(), "Blue".to_string() );
		self.add_filament(  "Pokymaker PLA-HT".to_string(), "Polymaker".to_string(),"PLA-HT".to_string(), "Red".to_string() );
		self.add_filament(  "Pokymaker PLA-HT".to_string(), 
		"Polymaker".to_string(),"PLA-HT".to_string(), "Green".to_string() );
	}
	
	fn list_inventory( &self ){
		println!( "Listing filaments in {}", self.library_name);
		for (pos, item ) in self.all_filament.iter().enumerate() {
			println!("[{}]: {} - {} - {} - {}", pos, item.name, item.manufacturer, item.material, item.color );
		}
	}

	fn get_inventory_copy( &self ) -> Vec<Filament>{
		self.all_filament.clone()
	}
	
	fn maybe_add_filament( &mut self ){
		println!("Let's add some filament.");
		println!( "You have {} items in your current inventory.", self.all_filament.len() );

		let mut filament_name = String::new();
		let mut filament_manufacturer = String::new();
		let mut filament_material = String::new();
		let mut filament_color = String::new();
		let mut spool_count_string = String::new();

		let mut filament_name_loop_completed = false;
		while !filament_name_loop_completed {
			println!("What will this filament be called?");
			match stdin().read_line(&mut filament_name) {
				Ok(_n) => 
					{
						println!("You entered: {}", filament_name.trim());
						filament_name_loop_completed = true;
					}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}
			
			
		}

		let mut filament_manufacturer_loop_completed = false;
		while !filament_manufacturer_loop_completed {
			println!("Who manufacturers this filament?");
			match stdin().read_line(&mut filament_manufacturer) {
				Ok(_n) => 
					{
						filament_manufacturer = filament_manufacturer.trim().to_string();
						println!("You entered: {}", filament_manufacturer);
						filament_manufacturer_loop_completed = true;
					}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}
			
			
		}

		let mut filament_material_loop_completed = false;
		while !filament_material_loop_completed {
			filament_material = String::new();
			println!("What material is this filament? Select a number of the materials, or type a filament type.");
			list_filament_types();
			let filament_materials = get_filament_materials();
			
			let mut input = String::new();

			println!("Enter a number (int or float):");
			println!("You entered: {}", input);

			io::stdin().read_line(&mut input).expect("X");

			match input.trim().parse::<usize>(){
				Ok( intnum ) => {
					filament_material = filament_materials[intnum].to_string();
					println!("You entered a number: {intnum}, which is {filament_material}.");
					filament_material_loop_completed = true;
					/*
					if in_filament_materials( filament_material.trim().to_string().clone() ){
						//let test = filament_materials.get(intnum as usize);
						let test = filament_materials[intnum];
						println!( "{}", test );
					}*/
				}
				Err(error) => {
					println!( "Invalid input. Error: {error}." );
					println!("{}", input.trim());
					if in_filament_materials( input.trim().to_string().clone() ){
						
						filament_material = input.trim().to_string();
						filament_material_loop_completed = true;
					} else {
						println!( "Invalid input. Error: {error}. Please try again." );
					}
				}
			}
			drop( input );
			println!("Material is: {filament_material}");
			/*
			match stdin().read_line(&mut filament_material) {
				Ok(_n) => 
					{
						println!("You entered: {}", filament_material.trim());
						if in_filament_materials( filament_material.trim().to_string().clone() ){
							filament_material_loop_completed = true;
						} else {
							filament_material = filament_material.trim().to_string();
							println!("{filament_material} is not a valid filament material. Please try again.")
						}
						
					}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}
			
			*/
		}

		let mut filament_color_loop_completed = false;
		while !filament_color_loop_completed {
			
		println!("What color is this filament?");
			match stdin().read_line(&mut filament_color) {
				Ok(_n) => 
					{
						println!("You entered: {}", filament_color.trim());
						filament_color_loop_completed = true;
					}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}
			
			
		}

		let mut filament_spool_count_loop_completed = false;
		while !filament_spool_count_loop_completed {
			
			println!("How many spools would you like to ad?");
			match stdin().read_line(&mut spool_count_string) {
				Ok(_n) => 
					{
						println!("You entered: {}", spool_count_string.trim());
						filament_spool_count_loop_completed = true;
					}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}
			
			
		}
		
		 match spool_count_string.trim() .parse::<i32>(){
			Ok(spool_count) => println!("User number is: {}", spool_count),
			Err(_) => println!("Something is not wrong. Maybe you did not enter a valid integer.")
		 }
		
		//let mut spool_count spool_count_string.parse::().unwrap();
		
		/*let new_filament = Filament {
			name: String::from(filament_name.trim()),
			filament_type: String::from(filament_material.trim()),
			color: String::from(filament_color.trim()),
			//spool_count: spool_count,
		};*/
		self.add_filament(filament_name.clone(), filament_material.clone(),filament_manufacturer.clone(), filament_color.clone() );


		//filaventory.push(new_filament);
		//println!("Added {} to inventory.", new_filament.);
		
		println!( "You NOW have {} items in your current inventory.", self.all_filament.len() );
	}

	fn add_filament( &mut self, filament_name:String, filament_menufacturer:String, filament_material:String, filament_color:String  ){
		println!("Adding Filament to Inventory");

		if in_filament_materials(filament_material.clone() ){
			let new_filament = Filament {
				name: String::from(filament_name.trim()),
				manufacturer: filament_menufacturer.trim().to_string(),
				material: String::from(filament_material.trim()),
				color: String::from(filament_color.trim()),
				//spool_count: spool_count,
			};
			self.all_filament.push(new_filament);
		}
		self.autosave_filament_library_json();
	}

	fn is_filament_allowed( &self ){
		println!("What string am I testing?");
		let mut test_filament_material = String::new();
		match stdin().read_line(&mut test_filament_material) {
				Ok(_n) => 
					{
						println!("You entered: {}", test_filament_material.trim());
						test_filament_material = test_filament_material.as_str().trim().to_string();
						
						let rez = in_filament_materials( test_filament_material );
						println!("{rez}");
					}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}

	}

	
	fn maybe_remove_filament( &mut self ){
		self.list_inventory( );
		println!( "Which filament would you like to remove?" );
		let mut filament_to_remove = String::new();
		stdin().read_line(&mut filament_to_remove).unwrap();
		match filament_to_remove.trim().parse::<u32>(){
			//Ok(index_pos) => self.removeFilament( &self, &index_pos),
			Ok(index_pos) => self.remove_filament( index_pos )
			,
			Err(_) => println!("Something is not wrong. Maybe you did not enter a valid integer.")
		}
	}
	
	fn remove_filament( &mut self, index_pos:u32 ){
		println!("Starting actual removal");
		println!( "Index POS: {}", index_pos);
		println!("Removing Filament from Inventory");
		
		//let test = self.all_filament.clone().into_iter().nth(index_pos.try_into().unwrap());
		let test = self.all_filament.get(index_pos as usize);
		println!( "{}", test.unwrap().name );
		
		//let removed_filament = self.all_filament[index_pos];
		
		self.all_filament.remove(index_pos.try_into().unwrap());
		//println!( "Removed {}", removed_filament.name );
		self.list_inventory( );
		
	}
	
	
}




fn main() {
	println!("##########DEBUG INFO##########");

	let args: Vec<String> = env::args().collect();
    dbg!(args.clone() );

	println!( "There are {} args.", args.len() );
	
	for (pos,args) in args.iter().enumerate() {
		println!( "[{pos}]: {}", args );
	}
	println!("##############################");

	println!( "Welcome to Filament Managr.");
	let mut main_menu = BTreeMap::new();
	main_menu.insert("o", "Open Existing Filament Library");
	main_menu.insert("c", "Create Filament Library");
	main_menu.insert("q", "Quit");

	let mut library_name = String::new();
	loop {
		println!("Get started.");
		let mut main_menu_selection = String::new();
		for (key, value) in &main_menu {
			println!("[{key}]: {value}");
			
		}

		match stdin().read_line(&mut main_menu_selection) {
				Ok(_n) =>  {
					/*
					println!("You entered: {}", input);
					dbg!( &input );
					dbg!( &input.trim() );
					*/
					match main_menu_selection.as_str().trim() {
						"o" => {
							println!("Select a Filament Library to Open");
							get_app_data_directory();
							get_app_default_save_directory_files();
							
						},
						"c" => {
							println!("What should we call this library?");
							
							match stdin().read_line(&mut library_name) {
								Ok(_n) => {
									library_name = library_name.trim().to_string();
									
									
								}
								Err(_) => todo!()
							}

						},
						"q"  => {

						},
						&_ => println!("Error"),
					}

				},
				Err(_) => todo!()
		}
		
		let mut inventory = FilamentLibrary::new( library_name.clone() );
		inventory.create_library_file();
	
		
		println!( "Library {} has {} items.", inventory.library_name, inventory.all_filament.len() );
		
		let mut commands = BTreeMap::new();
		commands.insert("1", "List inventory");
		commands.insert("2", "Add filament to inventory");
		commands.insert("3", "Delete filament to inventory");
		commands.insert("9", "List filament materials");
		commands.insert("t", "Test filament material names");
		commands.insert("e", "Export filament inventory to CSV");
		commands.insert("x", "Setup example filaments");
		commands.insert("h", "Help");
		commands.insert("q", "Quit");
		commands.insert("m", "Return to main menu");
		//inventory.push(
		loop {
			let mut input = String::new();
			println!("What would you like to do? Enter the character or character inside brackets to select an option.");
			for (key, value) in &commands {
				println!("[{key}]: {value}");
				
			}
			match stdin().read_line(&mut input) {
				Ok(_n) =>  {
					/*
					println!("You entered: {}", input);
					dbg!( &input );
					dbg!( &input.trim() );
					*/
					match input.as_str().trim() {
						//"1" => listFilament( &inventory ),
						"1" => inventory.list_inventory(),
						//"2" => addFilamentToCollection( &mut inventory ),
						"2" => inventory.maybe_add_filament(),
						//"3" => maybeRemoveFilament( &mut inventory ),
						"3" => inventory.maybe_remove_filament(),
						"9" => list_filament_types( ), 
						"0" => {
							println!("Goodbye");
							process::exit(0);
						}
						"x" => inventory.import_test_filaments(),
						"e" => inventory.export_csv().expect("FAIL"),
						//"j" => inventory.export_json(),
						"i" => inventory.import_csv().expect("test"),
						"t" => inventory.is_filament_allowed(),
						"m" => break(),
						&_ => println!("Error"),
					}
				}
				Err(error) => println!( "Invalid input. Error: {error}." )
			}
		}
	}
}

fn get_filament_materials() -> [&'static str; 11] {
	["PLA","PLA-HT","PETG","ASA","PCTG","NYLON","TPE","TPU","PET","PVA","PP"]
}

fn in_filament_materials( filament_material:String)  -> bool {
	println!("{filament_material}");
	let filament_materials = get_filament_materials();
	filament_materials.iter().any(|&i| i== filament_material)
}

fn list_filament_types( ) {
	println!("Listing Filament Materials");
	let filement_materials = get_filament_materials();
	println!( "There are {} filament types.", filement_materials.len() );
	
	for (pos,filement_materials) in filement_materials.iter().enumerate() {
		println!( "[{pos}]: {}", filement_materials );
	}
	
}

fn get_app_data_directory() -> PathBuf {
	let app_data = AppData::new("FilamentManagr");
	let data_dir = app_data.ensure_data_dir().unwrap();
	data_dir
}

fn get_app_autosave_directory() -> PathBuf{
	let mut data_dir = get_app_data_directory();
	data_dir.push("autosave" );
	data_dir
}

fn get_app_save_directory() -> PathBuf {
	get_app_default_save_directory()
}

fn get_app_default_save_directory() -> PathBuf{
	let mut data_dir = get_app_data_directory();
	data_dir.push("libraries" );
	data_dir
}

fn get_app_default_save_directory_files() -> Result<(), Box<dyn std::error::Error>>{
	let mut data_dir = get_app_data_directory();
	data_dir.push("libraries" );
	if data_dir.is_dir() {
		for entry in fs::read_dir(data_dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{}", entry.path().display());
        }
	}
	Ok(())
}



#[cfg(test)]
mod tests {
	 #[test]
    fn test_add_single_filament() {
		use crate::FilamentLibrary;
		let mut inventory = FilamentLibrary::new();
		inventory.add_filament( "Overture PLA Pro".to_string(), "Overture".to_string(), "PLA".to_string(), "Black".to_string() );
		let all_filament = inventory.get_inventory_copy();
		
		assert_eq!(all_filament.len(),1);
    }

}

/*
fn listFilament( inventory:&FilamentLibrary ) {
	println!( "Listing Filament" );
	println!( "Inventory has {} items.", inventory.all_filament.len() );
	
	
	for (pos, item ) in inventory.all_filament.iter().enumerate() {
        println!("[{}]: {} - {} - {}", pos, item.name, item.filament_type, item.color );
    }
}
*/

/*
fn addFilamentToCollection( inventory:&mut FilamentLibrary ){
	println!("Let's add some filament.");
	println!( "You have {} items in your current inventory.", inventory.all_filament.len() );
	println!("What will this filament be called?");
	
	let mut filamentName = String::new();
	stdin().read_line(&mut filamentName);
	println!("You entered: {}", filamentName);

	let mut filamentMaterial = String::new();
	println!("What material is this filament?");
	stdin().read_line(&mut filamentMaterial);
	println!("You entered: {}", filamentMaterial);
	
	let mut filament_color = String::new();
	println!("What color is this filament?");
	stdin().read_line(&mut filament_color);
	println!("You entered: {}", filament_color);
	
	let mut spool_count_string = String::new();
	
	println!("How many spools would you like to ad?");
	stdin().read_line(&mut spool_count_string);
	println!("You entered: {}", spool_count_string);
	
	 match spool_count_string.trim() .parse::<i32>(){
        Ok(spool_count) => println!("User number is: {}", spool_count),
        Err(_) => println!("Something is not wrong. Maybe you did not enter a valid integer.")
	 }
	
	//let mut spool_count spool_count_string.parse::().unwrap();
	
	let new_filament = Filament {
		name: String::from(filamentName.trim()),
		filament_type: String::from(filamentName.trim()),
		color: String::from(filament_color.trim()),
		//spool_count: spool_count,
	};
	
	//filaventory.push(new_filament);
	inventory.all_filament.push(new_filament);
	println!( "You NOW have {} items in your current inventory.", inventory.all_filament.len() );
}
*/
/*
fn maybeRemoveFilament( inventory:&mut FilamentLibrary ){
	
}

fn removeFilamentFromCollection( inventory:&mut FilamentLibrary ){
	let _removed_filament = inventory;
	//inventory.remove(index_pos);
	//println!( "Removed {}", removed_filament.name ); 
}
*/
/*
fn removeSpoolFromFilamentCollection( ){
	
}

fn addSpoolFromFilamentCollection( ){
	
}
*/

/*
struct Filament {
	name: String;
	
}
*/

/*
fn removeFilament( inventory ){
	println!("Starting Remove Filament");
}

fn addFilament( inventory ){
	println!("Starting Add Filament");
	
}
*/