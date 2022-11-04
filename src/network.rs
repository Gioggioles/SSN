use crate::{layer::Layer, lifNN::Neuron};
use std::ptr::null;
use ndarray::{Array2};

#[derive(Clone, Debug)]

pub struct Network<Layer> { //ricordare di aggiungere un altro ':' quando si crea la libreria

pub(crate) layers: Vec<Layer>,

pub(crate) num_layers : i64,
}

impl Network<Layer<'_, Neuron>>{
    pub fn new(num_layers: i64, neurons: Vec<Vec<Neuron>>, interlayer_weights: Array2<f64>, intralayer_weights: Array2<f64>) -> Network<Layer<'static, Neuron>> {
        let &mut layer_prec = null;
        let layers_new = Vec::new();
        for n in 0..num_layers{
           let l = Layer::new(neurons.get(n).unwrap(), intralayer_weights, interlayer_weights, layer_prec);
           layers_new.append(l);
           layer_prec = l;
        }
        
        Network{
            layers : layers_new,
            num_layers : num_layers
         } 
        
    }
    
}