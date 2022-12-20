use ndarray::Array2;
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug)]

pub struct Layer<Neuron> {
    pub(crate) neuroni: Vec<Neuron>, // List of all neurons in this layer
    
    pub(crate) interlayer_weights: Array2<f64>, // Matrix of the input weights (between neurons belonging to different layers). For the first layer, this must be a square diagonal matrix.
    
    pub(crate) intralayer_weights: Array2<f64>, // Square matrix of the intra-layer weights (between neurons belonging to the same layer)
    
    pub(crate) internal_spike: Vec<f64>, // Current value of spike for each Neuron of this layer

    pub(crate) ts_prec: f64, // Time of previous update 
}

impl<Neuron> Layer<Neuron> {
    pub fn new(neurons : Vec<Neuron>, intra_w : Array2<f64>, inter_w : Array2<f64>) -> Self{
        Self{
            internal_spike: vec![0.0; neurons.len()],
            neuroni : neurons,
            interlayer_weights : inter_w,
            intralayer_weights : intra_w,            
            ts_prec : 0.0 
        }
    }

    pub fn num_neuroni(&self) -> usize {
        self.neuroni.len()
    }

    pub fn get_neuroni_mut(&mut self, neuroni: usize) -> Option<&mut Neuron> {
        self.neuroni.get_mut(neuroni)
    }

    pub fn get_intralayer_weight(&self, row: usize, coloumn: usize) -> Option<&f64> {
        self.intralayer_weights.get((row, coloumn))
    }

    pub fn get_interlayer_weight(&self, row: usize, coloumn: usize) -> Option<&f64> {
        self.interlayer_weights.get((row, coloumn))
    }
    pub fn get_decadenza_internal_spike(&mut self, t_s: f64) -> Vec<f64>{ 
        let mut vt = Vec::new();
        let internal_temp = Arc::new(Mutex::new(Vec::from(self.internal_spike.clone())));
        let t_pr = self.ts_prec;
        for n in 0..self.internal_spike.len(){ 
            let internal_temp = internal_temp.clone();     
            vt.push(std::thread::spawn(move ||{
                let temp = internal_temp.lock().unwrap();
                let t = temp.get(n).unwrap() *((t_pr-t_s)/* moltiplicare per una lambda */).exp();
                drop(temp);
                internal_temp.lock().unwrap()[n] = t;
            }));
        }
        for v in vt{
            v.join().unwrap();
        } 

        self.internal_spike = internal_temp.lock().unwrap().to_vec();
        self.ts_prec = t_s;

        return self.internal_spike.clone()
    }
}