//Stats Project

fn count(v: &Vec<usize>) -> usize {
    return v.len();
}

fn sum(v: &Vec<usize>) -> usize {
    assert!(v.len() > 0);
    let mut sum = 0;

    for i in v {
        sum = sum + i;
    }

    return sum;
}

fn mean(v: &Vec<usize>) -> f32 {
    assert!(v.len() > 0);
    let mut sum = 0;

    for i in v {
        sum = sum + i;
    }

    let num = v.len() as f32;
    let mean:f32 = sum as f32 / num as f32;

    return mean;
}

fn median(v: &mut Vec<usize>) -> f32 {
    assert!(v.len() > 0);
    v.sort();

    let median;

    if v.len() % 2 == 1 {
        let index = v.len() / 2;
        median = v[index] as f32;
    } else {
        let indexright = v.len() / 2;
        let indexleft = indexright - 1;
        median = (v[indexright] + v[indexleft]) as f32 / 2 as f32;
    }

    return median;
}

fn mode(v: &mut Vec<usize>) -> usize {
    assert!(v.len() > 0);
    v.sort();

    let mut cur = v[0];
    let mut countcur = 0;
    let mut mode= v[0];
    let mut countmode = 0;

    for i in v {
        if *i == cur {
            countcur += 1;
        } else {
            if countcur > countmode {
                countmode = countcur;
                mode = cur;
            }
            cur = *i;
            countcur = 0;
        }
    }

    return mode;
}


fn min(v: &mut Vec<usize>) -> usize {
    assert!(v.len() > 0);
    v.sort();
    return v[0];
}


fn max(v: &mut Vec<usize>) -> usize {
    assert!(v.len() > 0);
    v.sort();
    return v[v.len()-1];
}

fn stdev(v: &mut Vec<usize>) -> f32 {
    assert!(v.len() > 1);
    v.sort();

    let mean = mean(v);
    let mut meansum: f32 = 0.0;
    let count = v.len();

    for i in v {
        meansum = meansum + ((*i as f32 - mean)*(*i as f32 - mean));
    }

    let stdev = meansum / (count as f32 -1.0);

    return stdev.sqrt();
}


fn percentile(v: &mut Vec<usize>, p: usize) -> f32 {
    assert!(p >= 0 && p <= 100 && v.len() != 0);
    v.sort();

    let percentile: f32;
    let percentage: f32 = p as f32 /100 as f32;

    if v.len() == 1 {
        percentile = v[0] as f32;
    } else if p == 100 {
        percentile = v[v.len()-1] as f32;
    } else {
        let total = v.len();
        let n = (percentage * (total - 1) as f32) as f32 + 1.0 as f32;
        let int = n.floor() as usize;
        let frac = n - int as f32;
        percentile = v[int - 1] as f32 + (frac * (v[int] - v[int - 1]) as f32);
    }

    return percentile;
}

use std::io;
use std::env;
use std::fs;

fn main() {

    println!("Select text file to read data from!");
    let mut file = String::new();
    io::stdin()
        .read_line(&mut file)
        .expect("Incorrect file name!");


    let datafile = fs::read_to_string(file.trim())
        .expect("Failed to read data file!");

    println!("data {}", datafile);

    let mut data:Vec<usize> = Vec::new();
    for i in datafile.chars() {
        if i != ' ' {
            let num = i.to_digit(10).unwrap();
            data.push(num as usize);
            println!("{}",num);
        }
    }
    // let mut data:Vec<usize> = Vec::new();
    // data.push(6);
    // data.push(7);
    // data.push(8);
    // data.push(9);
    // data.push(10);

    println!("count = {}",count(&data));
    println!("sum = {}",sum(&data));
    println!("mean = {}",mean(&data));
    println!("stdev = {}",stdev(&mut data));
    println!("median = {}",median(&mut data));
    println!("mode = {}",mode(&mut data));
    println!("min = {}",min(&mut data));
    println!("max = {}",max(&mut data));
    println!("  0th percentile = {}",percentile(&mut data, 0));
    println!(" 25th percentile = {}",percentile(&mut data, 25));
    println!(" 50th percentile = {}",percentile(&mut data, 50));
    println!(" 75th percentile = {}",percentile(&mut data, 75));
    println!("100th percentile = {}",percentile(&mut data, 100));


    // assert!(count(&data) == 5);
    // assert!(sum(&data) == 40);
    // assert!(mean(&data) == 8.0);
    // assert!(median(&mut data) == 8.0);
    // assert!(min(&mut data) == 6);
    // assert!(max(&mut data) == 10);
    // assert!(mode(&mut data) == 6);
    // assert!(stdev(&mut data) == 1.5811388);
    // assert!(percentile(&mut data, 0) == 6.0);
    // assert!(percentile(&mut data, 25) == 7.0);
    // assert!(percentile(&mut data, 50) == 8.0);
    // assert!(percentile(&mut data, 75) == 9.0);
    // assert!(percentile(&mut data, 100) == 10.0);
    //
    // println!("Passed!")

}
