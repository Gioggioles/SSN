//use std::ptr::null;
use ndarray::{Array2};

#[derive(Clone, Debug)]

pub struct Layer<Neuron> { //ricordare di aggiungere un altro ':' quando si crea la libreria
    /// List of all neurons in this layer
    pub(crate) neuroni: Vec<Neuron>,
    /// Matrix of the input weights (between neurons belonging to different layers). For the first layer, this must be a square diagonal matrix.
    pub(crate) interlayer_weights: Array2<f64>,
    /// Square matrix of the intra-layer weights (between neurons belonging to the same layer)
    pub(crate) intralayer_weights: Array2<f64>,
    // layer precedente
    //pub(crate) prec_layer: &Layer<Neuron>,  //Pensare di levare tale Layer e riscrivere la funzione aggiorna neuroni dentro Network
    //vec t-1
    pub(crate) internal_spike: Vec<f64>,

    pub(crate) ts_prec: f64,
}



impl<Neuron> Layer<Neuron> {

    pub fn new(neurons : Vec<Neuron>, intra_w : Array2<f64>, inter_w : Array2<f64>, /*layer_p : Option<Layer<Neuron>>*/) -> Self{
        Self{
            neuroni : neurons,
            interlayer_weights : inter_w,
            intralayer_weights : intra_w,
            internal_spike: Vec::new(), 
            ts_prec : 0.0
            //prec_layer : &layer_p.unwrap()                 
            //salvare vettore di spike all'interno del layer calcolato nel tempo precedente
        }
    }

    pub fn num_neuroni(&self) -> usize {
        self.neuroni.len()
    }

    pub fn get_neuroni_mut(&mut self, neuroni: usize) -> Option<&mut Neuron> {
        self.neuroni.get_mut(neuroni)
    }

    pub fn get_intralayer_weight(&self, row: usize, coloumn: usize) -> Option<f64> {
        self.intralayer_weights.get((row, coloumn)).copied()
    }

    pub fn get_interlayer_weight(&self, row: usize, coloumn: usize) -> Option<f64> {
        self.interlayer_weights.get((row, coloumn)).copied()
    }
    pub fn get_decadenza_internal_spike(&mut self, t_s: f64) -> Vec<f64>{
        for n in 0..self.internal_spike.len(){
            self.internal_spike[n] = self.internal_spike.get(n).unwrap()*((self.ts_prec-t_s)/* moltiplicare per una lambda */).exp()
        }
        self.ts_prec = t_s;

        return self.internal_spike
    }

}