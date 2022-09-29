fn main(){
    let mut groups = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];
    let test_member = "Susan";
    searchMember(groups,&test_member);
}

fn searchMember(groups:[[&str;4];6],member_name:&str){
    let mut in_group:bool = false;
    let mut group_number: Vec<i32> = Vec::with_capacity(6);
    let mut group_leader: Vec<i32> = Vec::with_capacity(6);
    for (group_num, v) in groups.iter().enumerate() {
        for (list_num,name) in v.iter().enumerate(){
            //println!("nums[{}]={}", pos, name.eq_ignore_ascii_case(member_name));
            if name.eq_ignore_ascii_case(member_name){
                in_group = true;
                if list_num == 0{
                    group_number.push(group_num as i32);
                    group_leader.push(group_num as i32);
                }
                else{
                    group_number.push(group_num as i32);
                }
            }
        }
    }
    println!("Whether the member in the group is {}",in_group);
    if in_group == false{
        return;
    }
    println!("This member is in group {:?}",group_number.into_iter().collect::<Vec<i32>>());
    let leader = group_leader.into_iter().collect::<Vec<i32>>();
    if leader.is_empty(){
        println!("This member is not a group leader");
    }
    else{
        println!("This member is the group leader of group {:?}",leader);
    }
    return;
}

