#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub struct PrefabRoom {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
    pub first_depth: i32,
    pub last_depth: i32,
}

#[allow(dead_code)]
pub const CHECKERBOARD: PrefabRoom = PrefabRoom {
    template: MAP_CHECKERBOARD,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
pub const SILLY_SMILE: PrefabRoom = PrefabRoom {
    template: MAP_SILLY_SMILE,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
pub const TOTALLY_NOT_A_TRAP: PrefabRoom = PrefabRoom {
    template: MAP_TOTALLY_NOT_A_TRAP,
    width: 5,
    height: 5,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
const MAP_CHECKERBOARD: &str = "
      
 #^#  
 g#%# 
 #!#  
 ^# # 
      
";

#[allow(dead_code)]
const MAP_SILLY_SMILE: &str = "
      
 ^  ^ 
  ##  
      
 #### 
      
";

#[allow(dead_code)]
const MAP_TOTALLY_NOT_A_TRAP: &str = "
     
 ^^^ 
 ^!^ 
 ^^^ 
     
";
