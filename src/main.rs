use lifNN::Neuron;

pub mod lifNN;
pub mod layer;
mod network;
use ndarray::{Array2};
use network::Network;
use ndarray::prelude::*;

pub fn main() {

     
    let mut neurons1 = Vec::new();
    let neurone_11 = Neuron::new(0.7, 0.5, 1.3, 1.0);
    let neurone_12 = Neuron::new(0.5, 0.4, 1.9, 1.0);
    let neurone_13 = Neuron::new(0.3, 0.3, 1.4, 1.0);
    neurons1.push(neurone_11);
    neurons1.push(neurone_12);
    neurons1.push(neurone_13);
    


    let mut neurons2 = Vec::new();
    let neurone21 = Neuron::new(0.8, 0.5, 1.7, 1.5);
    let neurone22 = Neuron::new(0.7, 0.8, 1.4, 1.6);

    neurons2.push(neurone21);
    neurons2.push(neurone22);
    
   

    let mut neurons3 = Vec::new();
    let neurone31 = Neuron::new(0.3, 0.5, 1.5, 1.0);
    let neurone32 = Neuron::new(0.1, 0.3, 1.6, 1.0);
    let neurone33 = Neuron::new(0.9, 0.5, 1.5, 1.5);

    neurons3.push(neurone31);
    neurons3.push(neurone32);
    neurons3.push(neurone33);


    let intra1: Array2::<f64> =  array![[0.0, 0.5, 0.2], [1.0, 0.0, 0.7], [0.1, 0.6, 0.0]];
    let inter1 =  Array2::from_shape_vec((3, 1), vec![1.0, 1.0, 1.0]).unwrap();



    
    let intra2: Array2::<f64> = array![[0.0, 0.9],[0.6, 0.0]]; //2x2
    let inter2: Array2::<f64> = array![[0.9, 1.1, 1.1], [0.5, 0.3, 1.3]]; //2x3


    let intra3: Array2::<f64> = array![[0.0, 0.7, 0.3],[0.2, 0.0, 0.8],[0.6, 0.3, 0.0]]; //3x3
    let inter3: Array2::<f64> = array![[0.2, 0.5], [0.9, 0.6], [0.7, 0.8]]; //3x2
    
    
    
    let mut network = Network::new();
    network.add_layer(neurons1, inter1, intra1);
    network.add_layer(neurons2, inter2, intra2);
    network.add_layer(neurons3, inter3, intra3);

   // print!("{}",network.num_layers);

    let spike_m: Array2::<f64> = array![[1.0, 0.0, 0.0]]; 

    //array![[1.0, 0.0, 0.0],[0.0, 1.0, 1.0],[1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]]; 

    let tempi = vec![1.0];

    //, 2.0, 3.0, 5.0, 6.0, 8.0, 11.0

    let mut count = 0;
    for ts in tempi{
        let stampa = network.aggiorna_neuroni(ts, spike_m.row(count).to_vec());
        count+=1;
        for  i in stampa{
            print!("{}", i);
        }
        print!("\n");
    }

    
}
