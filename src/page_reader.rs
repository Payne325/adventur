
struct PageResult {
  nextPage : u32,
  enemy : String
  // TODO: make this use the character type,
  // then we can use a factory to assign an enemy
}

struct PageData {
  text: String,
  nextPageOptions: Vec<u32>,
  battleInitiated: bool,
  enemy: String
}

fn read_page(page : u32) -> PageData {
  
  //Parse text file
  
  // page_text = get text from file
  // page_options = get_options_from_file
  // page_battle_initiated = get_battle_initiated_from_file
  // page_enemy = get_enemy_type_from_file
  
  /*
  PageData {
    text : page_text,
    nextPageOptions : page_options,
    battleInitiated : page_battle_initiated
    enemy : page_enemy
  }
  */  
}

fn process_page(page : PageData) -> PageResult {
  
  println!("{}", page.text);
  
  //handle user input
  let mut selection_str = String::new();
		
	io::stdin().read_line(&mut selection_str)
		.expect("Failed to read line!");
  
  //Construct and return some object indicating result will trigger a battle.
  if page.battleInitiated == true {
    let enemy = Character {
      name: String::from("TestingFileReader"),
      skill: 0,
      stamina: 0,
      luck: 0,
      provisions: 0
    }		
    
    PageResult {
      nextPage : page.page_options[0],
      enemy : enemy
    }
  }
  
  let selection = selection_str.trim().parse::<u32>().unwrap();
  
  //Construct and return some object indicating the next page to be read.
  if page.nextPageOptions.contains(selection) {  
    let enemy = Character {
      name: String::from("TestingFileReader"),
      skill: 0,
      stamina: 0,
      luck: 0,
      provisions: 0
    }		
    
    PageResult {
      nextPage : selection,
      enemy : enemy
    }
  }
  else {
    //If the user gives invalid input, 
    //then just recursively call this function until valid input is given.
    println!("\n\n\nInvalid Option!\n\n\n");
    process_page(page);
  }
}