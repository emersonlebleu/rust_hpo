#![allow(unused_imports)]
use hpo::HpoTermId;
use hpo::similarity::{Similarity, StandardCombiner, GroupSimilarity};
use::hpo::{Ontology, HpoSet};
use::hpo::term::HpoGroup;
mod custom_jaccard;

fn main() {
    //the first list of HPO terms
    let hpo_ids1 = "HP:0001188,HP:0001252,HP:0001263,HP:0001276,HP:0001290,HP:0001324,HP:0001332,HP:0002015,HP:0002058,HP:0002072,HP:0002134,HP:0002355,HP:0002376,HP:0003701,HP:0004305,HP:0006789,HP:0007183,HP:0010862";
    //the second list of HPO terms
    let hpo_ids2 = "HP:0001252,HP:0001263,HP:0001290,HP:0002334,HP:0011827";
    let hpo_ids3 = "HP:0000023,HP:0000218,HP:0000473,HP:0000565,HP:0000582,HP:0000750,HP:0001057,HP:0001263,HP:0001290,HP:0001332,HP:0001344,HP:0001655,HP:0002015,HP:0002020,HP:0002134,HP:0002179,HP:0002194,HP:0002376,HP:0002870,HP:0003236,HP:0006466,HP:0006528,HP:0007048,HP:0007183,HP:0007325,HP:0010296,HP:0010862,HP:0010864,HP:0011344,HP:0011649,HP:0012450,HP:0012736,HP:0012751,HP:0012785,HP:0025335,HP:0025336,HP:0031936";
    
    //split the lists into vectors
    let hpo_ids1: Vec<u32> = hpo_ids1.split(",").filter_map(|x| {
        x.trim_start_matches("HP:").parse::<u32>().ok()
    }).collect();
    
    let hpo_ids2: Vec<u32> = hpo_ids2.split(",").filter_map(|x| {
        x.trim_start_matches("HP:").parse::<u32>().ok()
    }).collect();
    
    let hpo_ids3: Vec<u32> = hpo_ids3.split(",").filter_map(|x| {
        x.trim_start_matches("HP:").parse::<u32>().ok()
    }).collect();

    //start a timer to see how long it takes to load the ontology
    let start = std::time::Instant::now();
    //allow this to not be used
    #[allow(unused_variables)]
    let ontology = Ontology::from_binary("bin_hpo_file").unwrap();
    //stop the timer
    let duration1 = start.elapsed();
    //load the ontology
    println!("Ontology loaded: {:?}", duration1); 

    //Create a group from the hpo_ids1 vector
    let hpo_group1 = HpoGroup::from(hpo_ids1);
    let hpo_set1 = HpoSet::new(&ontology, hpo_group1);
    //Create a group from the hpo_ids2 vector
    let hpo_group2 = HpoGroup::from(hpo_ids2);
    let hpo_set2 = HpoSet::new(&ontology, hpo_group2);
    //Create a group from the hpo_ids3 vector
    let hpo_group3 = HpoGroup::from(hpo_ids3);
    let hpo_set3 = HpoSet::new(&ontology, hpo_group3);

    let sim = GroupSimilarity::new(StandardCombiner::default(), custom_jaccard::CustomJaccard{});
    //start a timer to see how long it takes to calculate the first
    let start = std::time::Instant::now();
    let similarity1_2 = sim.calculate(&hpo_set1, &hpo_set2);
    //stop the timer
    let duration2 = start.elapsed();
    println!("Similarity #1-#2: {similarity1_2} time: {:?}", duration2);

    //start a timer to see how long it takes to calculate the second
    let start = std::time::Instant::now();
    let similarity1_3 = sim.calculate(&hpo_set1, &hpo_set3);
    //stop the timer
    let duration3 = start.elapsed();
    println!("Similarity #1-#3: {similarity1_3} time: {:?}", duration3);
}