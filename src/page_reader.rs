use std::fs;

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
  let filename = concat!(page.to_string(), ".txt");
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the page!");
   
  let contentVec: Vec<&str> = contents.split("*").collect();
  
  page_options: Vec<i32> = 
    contentVec[1].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
  
  page_battle_initiated = if contentVec[2] == True { true } else { false }
  
  PageData {
    text : contentVec[0],
    nextPageOptions : page_options,
    battleInitiated : page_battle_initiated
    enemy : contentVec[3]
  }
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
  
  if page.nextPageOptions.len() == 0 {
    //GAME OVER.
    
    let enemy = Character {
      name: String::from("TestingFileReader"),
      skill: 0,
      stamina: 0,
      luck: 0,
      provisions: 0
    }		
    
    PageResult {
      nextPage : -1,
      enemy : enemy
    }
  }
  
  //Construct and return some object indicating the next page to be read.
  else if page.nextPageOptions.contains(selection) {  
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