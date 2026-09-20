#![allow(unused)]
#![allow(unused_imports)]
use core::error;
use std::collections::hash_map;
use std::hash::Hash;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::{
	error::Error,
	io,
	io::Read,
	io::stdin,
	io::Write,
	io::BufReader,
	fs,
	fs::File,
	fs::OpenOptions,
	process,
	env,
	collections::BTreeMap,
	collections::HashMap,
	path::Path,
};//use serde_ };
use app_data::AppData;
use hex_color::HexColor;
use uuid::Uuid;
use indexmap::{IndexMap, IndexSet};
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

#[derive(Clone, Debug,serde::Serialize, serde::Deserialize)]
struct FilamentLibrary {
	pub library_name:String,
	pub id: Uuid,
	pub all_filament:Vec<Filament>,
	recycle_bin:Vec<Filament>,
	manufacturers:Vec<Filament>,
	filament_material:Vec<Filament>,
}


#[derive(Clone, Debug,serde::Serialize, serde::Deserialize)]
struct Filament {
	name: String,
	manufacturer: String,
	material: String,
	series: Option<String>,
	favorite:Option<bool>,
	spools:Vec<Spool>,
	disabled:Option<bool>,
	hidden:Option<bool>
	//spool_count: i8,
}

impl Filament {
	fn new( name:String, manufacturer:String,material:String) -> Self {
		Self {
			name:name,
			manufacturer:manufacturer,
			material:material,
			series:None,
			favorite: None,
			spools: Vec::new(),
			disabled: Some(false),
			hidden: None,
		}
	}
}
/*
impl Default for Filament {
	fn default() -> Self {
		Filament { 
			name:String::new(),
			manufacturer:String::new(),
			material:String::new(),
			series:Some(String::new()),
			favorite: None,
			spools:Vec::new(),
			disabled: None,
			hidden: None,
		}
	}
}
*/
#[derive(Clone, Debug,serde::Serialize, serde::Deserialize)]
struct Spool {
	material: String,
	color_name: String,
	hex: Option<HexColor>,
	weight: i16,
	price:Option<f32>,	//spool_count: i8,
}

impl Spool {
	fn new( material: String, color_name: String, hex_coolor:String, price:f32) -> Self {
		Self {
			material:material,
			color_name:color_name,
			hex:None,
			weight: 1000,
			price:Some(price),
		}
	}
}
/*
impl Default for Spool {
	fn default() -> Self {
		Spool {
			color_name:Option::None,
			hex:Option::HexColor,
			weight:1000,
			price:0.0,
		}
	}
}
*/

impl FilamentLibrary {
	fn new( name: String) -> Self {
		Self {
			library_name:name,
			id: Uuid::new_v4(),
			all_filament:Vec::new(),
			recycle_bin:Vec::new(),
			manufacturers:Vec::new(),
			filament_material:Vec::new(),
		}
	}
	
	fn create_list_of_manufactuers(){

	}

	fn get_library_sourcefile_path(&self) -> PathBuf {
		let mut app_save_directory = get_app_save_directory();
		let mut sourcefile_full_path = app_save_directory;
		sourcefile_full_path.push(self.id.to_string());
		sourcefile_full_path.set_extension("json");
		sourcefile_full_path
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
				let mut filament_library_json:String = serde_json::to_string( &self ).expect("CONVERT TO JSON FAILED");
				
				writeln!(&mut file,"{}", filament_library_json ).expect("JSON WRITE FAILED");
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
			wtr.write_record(["Filament Name","Filament Menufacturer", "Filament Material","Favorite Filament?", "Filament Color","Filament Hex code","Disabled?","Hidden?","Spool Count"])?;
			for ( item ) in self.all_filament.iter() {
				wtr.write_record([item.name.clone(), item.manufacturer.clone(), item.material.clone() ])?;
			}
			wtr.flush()?;
			Ok(())
	}

	fn save_default( &self ){
		let sourcefile_full_path:PathBuf = self.get_library_sourcefile_path();

		let sourcefile = OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(false)
			.open(sourcefile_full_path.clone());

		match sourcefile {
			Ok(mut file) => {
				let mut filament_library_json:String = serde_json::to_string( &self ).expect("CONVERT TO JSON FAILED");
				
				writeln!(&mut file,"{}", filament_library_json ).expect("JSON WRITE FAILED");
			},
			Err(error) => {
				println!("Crap!");
				println!("{}",error);
				println!("{}",sourcefile_full_path.clone().display() );
			}
		}
	}

	fn export_json( &self ){

	}

	fn prepare_instance_for_json_export( &self ){
		

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
		/*
		self.add_filament( "Overture PLA Pro".to_string(), "Overture".to_string(), "PLA".to_string(), "Black".to_string() );
		self.add_filament(  "Overture PLA Pro".to_string(),"Overture".to_string(), "PLA".to_string(), "Blue".to_string() );
		self.add_filament( "Pokymaker PLA-HT".to_string(),"Polymaker".to_string(), "PLA-HT".to_string(), "Blue".to_string() );
		self.add_filament(  "Pokymaker PLA-HT".to_string(), "Polymaker".to_string(),"PLA-HT".to_string(), "Red".to_string() );
		self.add_filament(  "Pokymaker PLA-HT".to_string(), 
		"Polymaker".to_string(),"PLA-HT".to_string(), "Green".to_string() );
		*/

		self.add_filament( "Overture PLA Pro".to_string(), "Overture".to_string(), "PLA".to_string() );
		self.add_filament(  "Overture PLA Pro".to_string(),"Overture".to_string(), "PLA".to_string() );
		self.add_filament( "Pokymaker PLA-HT".to_string(),"Polymaker".to_string(), "PLA-HT".to_string() );
		self.add_filament(  "Pokymaker PLA-HT".to_string(), "Polymaker".to_string(),"PLA-HT".to_string() );
		self.add_filament(  "Pokymaker PLA-HT".to_string(), 
		"Polymaker".to_string(),"PLA-HT".to_string());
	}
	
	fn list_inventory( &self ){
		println!( "Listing filaments in {}", self.library_name);
		for (pos, item ) in self.all_filament.iter().enumerate() {
			println!("[{}]: {} - {} - {}", pos, item.name, item.manufacturer, item.material);
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
			println!("Who manufacturers this filament? Select the corresponding number, or type a filamanufacturer name.");
			list_filament_manufacturers();
			let filament_manufacturers = get_filament_manufacturers();

			let mut filament_manufacturer_user_input = String::new();

			io::stdin().read_line(&mut filament_manufacturer_user_input).expect("X");
			println!("You entered: {}", filament_manufacturer_user_input.trim());

			match filament_manufacturer_user_input.trim().parse::<usize>(){
				Ok( intnum ) => {
					filament_manufacturer = filament_manufacturers[intnum].to_string();
					println!("You entered a number: {intnum}, which is {filament_manufacturer}.");
					filament_manufacturer_loop_completed = true;
					/*
					if in_filament_materials( filament_material.trim().to_string().clone() ){
						//let test = filament_materials.get(intnum as usize);
						let test = filament_materials[intnum];
						println!( "{}", test );
					}*/
				}
				Err(error) => {
					println!( "Invalid input. Error: {error}." );
					println!("{}", filament_manufacturer_user_input.trim());
					if in_filament_manufacturers( filament_manufacturer_user_input.trim().to_string().clone() ){
						
						filament_manufacturer = filament_manufacturer_user_input.trim().to_string();
						filament_manufacturer_loop_completed = true;
					} else {
						println!( "Invalid input. Error: {error}. Please try again." );
					}
				}
			}
			
		}

		let mut filament_material_loop_completed = false;
		while !filament_material_loop_completed {
			filament_material = String::new();
			println!("What material is this filament? Select a number of the materials, or type a filament type.");
			list_filament_types();
			let filament_materials = get_filament_materials();
			
			let mut input = String::new();
			
			io::stdin().read_line(&mut input).expect("X");
			println!("You entered: {}", input);

			match input.trim().parse::<usize>(){
				Ok( intnum ) => {
					filament_material = filament_materials[intnum].to_string();
					println!("You entered a number: {intnum}, which is {filament_material}.");
					filament_material_loop_completed = true;
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
		
		}
		println!("Made it to here");
		
		self.add_filament(filament_name.clone(), filament_manufacturer.clone(),filament_material.clone());
		println!( "You NOW have {} items in your current inventory.", self.all_filament.len() );
	}

	fn add_filament( &mut self, filament_name:String, filament_manufacturer:String, filament_material:String  ){
		println!("Adding Filament to Inventory");

		if in_filament_materials(filament_material.clone() ){

			let mut new_filament = Filament::new( 
				String::from(filament_name.trim()),
				filament_manufacturer.trim().to_string(),
				String::from(filament_material.trim())
			);
			/*
			let new_filament = Filament::new{
				name: String::from(filament_name.trim()),
				manufacturer: filament_menufacturer.trim().to_string(),
				material: String::from(filament_material.trim()),
				//color: String::from(filament_color.trim()),
				//spool_count: spool_count,
			};
			*/
			self.all_filament.push(new_filament);
		}
		//self.autosave_filament_library_json();
	}

	fn maybe_add_spool( &self, filament_material:String ){
		let mut filament_name = String::new();
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
	}

	fn add_spool( &self ){

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
	let args: Vec<String> = env::args().collect();
	handle_args( args.clone() );
	println!( "Welcome to Filament Managr.");
	let mut library:FilamentLibrary = library_picker();
}

fn handle_args( args:Vec<String> ) {
	println!("##########DEBUG INFO##########");
	  dbg!(args.clone() );
	println!( "There are {} args.", args.len() );
	for (pos,args) in args.iter().enumerate() {
		println!( "[{pos}]: {}", args );
	}
	println!("##############################");
}


fn library_picker() -> FilamentLibrary {
	let mut main_menu = BTreeMap::new();
	main_menu.insert("o", "Open Existing Filament Library");
	main_menu.insert("c", "Create Filament Library");
	main_menu.insert("q", "Quit");

	let mut library_name = String::new();
	loop {
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
							//get_app_data_directory();
							//list_app_default_save_directory_files();
							let files = get_app_default_save_directory_files();
							if(files.is_empty()){
								println!("No libraries found.");
								library_picker();
							}

							for (pos,file) in files.iter().enumerate() {
								println!( "[{pos}]: {}", file.0 );
							}

							let mut library_selection_input:String = String::new();
							stdin().read_line(&mut library_selection_input);

							match library_selection_input.trim().parse::<usize>(){
								Ok( index ) =>{
									if let Some((key, value)) = files.get_index(index) {
										println!("Item at index x{}x: y{}y => z{:#?}z", index, key, value);

										let file = File::open(value.as_path() );
				
										match file {
											Ok(file) => {
												let mut reader = BufReader::new(file);
												let mut file_contents_json = String::new();
												reader.read_to_string(&mut file_contents_json);
												//println!("Buffered File");
												//println!("{}", file_contents_json);
												let inventory: FilamentLibrary = serde_json::from_str(&file_contents_json).unwrap();

												library_menu(inventory);
											},
											Err(_) => {
											todo!("File is not okay")
											}
										}
									} else {
										println!("Index out of bounds!");
									}	
								},
								Err(..) => todo!(),
							}
							
							
							
						},
						"c" => {
							println!("What should we call this library?");
							
							match stdin().read_line(&mut library_name) {
								Ok(_n) => {
									library_name = library_name.trim().to_string();
									let mut inventory = FilamentLibrary::new( library_name.clone() );
									inventory.create_library_file();
									library_menu(inventory.clone());
								}
								Err(_) => todo!()
							}

						},
						"q"  => {
							println!("Goodbye");
							process::exit(0);
						},
						&_ => println!("Error"),
					}
				},
				Err(_) => todo!()
		}
	}
}

fn library_menu( mut inventory: FilamentLibrary){
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
	commands.insert("s", "Save");
	commands.insert("m", "Return to main menu");
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
					"s" => inventory.save_default(),
					"m" => break(),
					&_ => println!("Error"),
				}
			}
			Err(error) => println!( "Invalid input. Error: {error}." )
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

fn get_filament_manufacturers() -> [&'static str; 20] {
	["Overture", "Polymaker", "BambuLab","FlashForge","Amolen","AnyCubic","Hatchbox","ProtoPasta","Sunlu","iSANMATE","eSUN","Eryone","Creality","Elegoo", "Amazon","Polar Filament","Siraya Tech","American Filament","Canadian Filaments","Generic"]
}

fn in_filament_manufacturers( filement_manufacturer:String)  -> bool {
	println!("{filement_manufacturer}");
	let filement_manufacturers = get_filament_manufacturers();
	filement_manufacturers.iter().any(|&i| i== filement_manufacturer)
}

fn list_filament_manufacturers( ) {
	println!("Listing Filament Manufacturers");
	let filement_manufacturers = get_filament_manufacturers();
	println!( "There are {} filement manufacturers.", filement_manufacturers.len() );
	
	for (pos,filement_manufacturer) in filement_manufacturers.iter().enumerate() {
		println!( "[{pos}]: {}", filement_manufacturer );
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

fn list_app_default_save_directory_files(){
	let files = get_app_default_save_directory_files();
	for (pos,file) in files.iter().enumerate() {
		println!( "[{pos}]: {}", file.0 );
	}
	//println!( "{:#?}", files );
	/*match files {
		Ok(saved_files:HashMap) => {
			println!("{:#?}",x);
			//for (pos,files) in files.iter().enumerate() {
			//	println!( "[{pos}]: {:#?}", files );
			//}
		},
		Err(_) => {
			todo!("File is not okay")
		}
	}*/
	
}

fn get_app_default_save_directory_files() -> IndexMap<String,PathBuf>{
	let mut data_dir = get_app_data_directory();
	data_dir.push("libraries" );
	//let mut libraries_vec: Vec =  Vec::new();
	let mut libraries_hash: IndexMap<String,PathBuf> = IndexMap::new();

	if data_dir.is_dir() {
		let dir =  fs::read_dir(data_dir);
		match dir {
			Ok(dir) => {
				let mut entries: Vec<PathBuf> = dir
				.filter(Result::is_ok)
				.map(|e| e.unwrap().path())
    		    .collect();

				for entry in entries {
				//println!( "{:#?}", entry );
				let path = entry.as_path();
				let filename = entry.file_name();
				//println!("{}", entry.file_name().display());
		
				let file = File::open(entry.as_path() );
				
				match file {
					Ok(file) => {
						let mut reader = BufReader::new(file);
						let mut file_contents_json = String::new();
						reader.read_to_string(&mut file_contents_json);
						//println!("Buffered File");
						//println!("{}", file_contents_json);
						let inventory: FilamentLibrary = serde_json::from_str(&file_contents_json).unwrap();
						//println!("{:#?}", inventory);
						//println!("{}", inventory.library_name);

						//let mut library_tuple = (String::new(),path::new());
						//let mut library_tuple = (inventory.library_name.to_string(),filename, path.clone());
						//libraries_vec.push(library_tuple);

						libraries_hash.insert(inventory.library_name.to_string(),path.to_path_buf());
						

					},
					Err(_) => {
						todo!("File is not okay")
					}
				}
				
				
			}
			},
			Err(_) => {
					todo!("File is not okay")
				}
			
		}

		 
		/*
		println!( "Vector of Tuples");
		println!("{:#?}",libraries_vec);
		println!( "Hash of Data");
		println!("{:#?}",libraries_hash);
		*/
		
	}
	return libraries_hash
}



#[cfg(test)]
mod tests {
	 #[test]
    fn test_add_single_filament() {
		use crate::FilamentLibrary;
		let mut inventory = FilamentLibrary::new();
		inventory.add_filament( "Overture PLA Pro".to_string(), "Overture".to_string(), "PLA".to_string() );
		let all_filament = inventory.get_inventory_copy();
		
		assert_eq!(all_filament.len(),1);
    }

}