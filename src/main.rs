use std::{env::args, io::{BufWriter, Write}};
use indicatif::ProgressBar;
use std::time::Duration;





fn main() {
    let argmnts = args().collect::<Vec<String>>();
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    
    // Below block for just running this script without commandArgs
    
    //let matrix = csv_to_linalg("/husky/douglas/sim/B_uniformis_no_to_all4plas_spectratest_260130__12_09/countsketch_mat.csv");
    //let mut out_costheta_file  = BufWriter::new(std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("/husky/douglas/sim/B_uniformis_no_to_all4plas_spectratest_260130__12_09/costheta.csv").expect("This path could NOT be used as output path. Maybe dir does not exist?"));
    //bar.finish_with_message("Mean cosine thetas computed for all cells!");
    ////////////////////////////////////////////////////////////////////////


    let matrix = csv_to_linalg(format!("/husky/douglas/sim/{}/countsketch_mat.csv", &argmnts[1]));
    let mut out_costheta_file  = BufWriter::new(std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(format!("/husky/douglas/sim/{}/costheta.csv", &argmnts[1])).expect("This path could NOT be used as output path. Maybe dir does not exist?"));
    let costheta_vec = cos_theta_vector(matrix);
    bar.finish_with_message("Mean cosine thetas computed for all cells!");

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    for i in costheta_vec {
        write!(out_costheta_file, "{}\n", i).expect("Problem writing the costheta info!");     
    }
    out_costheta_file.flush().expect("Problem flushing the costheta bufwriter!");
    bar.finish_with_message("Outfile csv written!");
}


fn csv_to_linalg(path: String) -> Vec<Vec<f64>> {

    
    let mut countsketchcsv = csv::ReaderBuilder::new()
                                        .has_headers(false)
                                        .from_path(path).expect("file could not be opened");
    let mut rows = Vec::new();
    for record in countsketchcsv.records() {
        let countsk_vec_for_spc: Vec<f64> = record
                                                .unwrap()
                                                .iter()
                                                .skip(2)
                                                .map(|s| s.parse::<f64>().unwrap())
                                                .collect::<Vec<f64>>();

        rows.push(countsk_vec_for_spc);
    }
    rows
}

fn cos_theta_vector(matrix: Vec<Vec<f64>>) -> Vec<f64>{
    matrix.iter().map(|row| {
        (&matrix.iter().map(|comp_row| {
            row.iter().zip(comp_row.iter()).map(|(x, y)| x * y).sum::<f64>() / (row.iter().map(|x| x*x).sum::<f64>().sqrt()*comp_row.iter().map(|x| x*x).sum::<f64>().sqrt())}).sum::<f64>()-1.0_f64) / (matrix.len()-1) as f64
        }).collect::<Vec<f64>>()
}