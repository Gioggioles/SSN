#[derive(Clone, Debug)]

pub struct Network<Layer> { //ricordare di aggiungere un altro ':' quando si crea la libreria

pub(crate) layers: Vec<Layer>,

pub(crate) num_layers : f64,
}

impl Network{
    pub fn new(num_layers: f64, neurons: Vec<Vec<Neuron>>, interlayer_weights: Array2<f64>, intralayer_weights: Array2<f64>) -> Network {
        &mut layer_prec = null;
        layers_new = Vec![];
        for n in 0..num_layers{
           l = Layer::new(neurons.get(n), intralayer_weights, interlayer_weights, layer_prec);
           layers_new.append(l);
           layer_prec = l;
        }
        
        Network{
            layers = layers_new,
            num_layers = num_layers
         } 
        
    }
    
}