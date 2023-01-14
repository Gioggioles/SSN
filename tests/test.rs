//pub use SSN::{lif::Neuron};
// use network::Network;
extern crate ssn;


#[cfg(test)]
mod decode_tests {
    use ndarray::{Array2, array};
    use ssn::Neuron;
    use ssn::Network;

    #[test]
    fn test_1() {
        let mut neurons1 = Vec::new();
        let neurone_11 = Neuron::new(0.6, 0.45, 1.03, 1.2); 
        let neurone_12 = Neuron::new(0.6, 0.4, 1.6, 1.1);
        neurons1.push(neurone_11);
        neurons1.push(neurone_12);
    
    
    
        let mut neurons2 = Vec::new();
        let neurone21 = Neuron::new(0.6, 0.53, 1.25, 1.2); 
        let neurone22 = Neuron::new(0.7, 0.5, 1.14, 1.1); 
        neurons2.push(neurone21);
        neurons2.push(neurone22);
    
         
        let mut neurons3 = Vec::new();
        let neurone31 = Neuron::new(0.77, 0.5, 1.2, 1.2);
        neurons3.push(neurone31);
    
    
        let intra1: Array2::<f64> =  array![[0.0, 0.7],[0.6, 0.0]];
        let inter1 =  Array2::from_shape_vec((2, 1), vec![1.0, 1.0]).unwrap();
    
        let intra2: Array2::<f64> = array![[0.0, 0.9],[0.8, 0.0]]; //2x2
        let inter2: Array2::<f64> = array![[0.7, 0.8], [0.9, 0.75]]; //2x2
    
        let intra3: Array2::<f64> = array![[0.0]]; //1x1
        let inter3: Array2::<f64> = array![[0.8, 0.9]]; //1x2
    
        let mut network = Network::new();
        network.add_layer(neurons1, inter1, intra1);
        network.add_layer(neurons2, inter2, intra2);
        network.add_layer(neurons3, inter3, intra3);
    
        let spike_m: Array2::<f64> = array![[1.0, 0.0]];
    
        let count = 0;
        let ts = 1.0; 
        let result = network.aggiorna_neuroni(ts, spike_m.row(count).to_vec());

    assert_eq!(result, [1.0]);

}
}