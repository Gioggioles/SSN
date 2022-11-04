use std::ptr::null;
use ndarray::{Array2};

#[derive(Clone, Debug)]

pub struct Layer<'a, Neuron> { //ricordare di aggiungere un altro ':' quando si crea la libreria
    /// List of all neurons in this layer
    pub(crate) neuroni: Vec<Neuron>,
    /// Matrix of the input weights (between neurons belonging to different layers). For the first layer, this must be a square diagonal matrix.
    pub(crate) interlayer_weights: Array2<f64>,
    /// Square matrix of the intra-layer weights (between neurons belonging to the same layer)
    pub(crate) intralayer_weights: Array2<f64>,
    // layer precedente
    pub(crate) prec_layer: &'a Layer<'a, Neuron>,  //Pensare di levare tale Layer e riscrivere la funzione aggiorna neuroni dentro Network
    //vec t-1
    pub(crate) internal_spike: Vec<f64>,
}



impl<Neuron> Layer<'_, Neuron> {

    pub fn new(neurons : Vec<Neuron>, intra_w : Array2<f64>, inter_w : Array2<f64>, layer_p : Option<Layer<Neuron>>) -> Self{
        Self{
            neuroni : neurons,
            interlayer_weights : inter_w,
            intralayer_weights : intra_w,
            internal_spike: Vec::new(), 
            prec_layer : &layer_p.unwrap()                 
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

    pub fn aggiorna_Neuroni (&mut self, ts : f64, spike : Vec<f64>) -> Vec<f64>{  //spike -> vettore di 0/1 dove ogni posizione corrisponde allo spike nel tempo ts del neurone nel layer precedente
        if (self.prec_layer == null){ //primo layer
            let s = Vec::new();
            //decadenza internal_spike
            spike = spike + self.internal_spike;
           
            for l in 0..spike.len(){  //controllo di spike, non può essere oltre 1
                if spike.get(l) > 1{
                    spike.get(l) = 1;
                }
            }
            for m in 0..self.num_neuroni(){  //controlli tutti i neuroni
                s.append(self.neuroni.get(m).unwrap().potential_evolution(spike.get(m),ts));
            }

            //Aggiornamento dei collegamenti intraLayer
            for n in 0..self.num_neuroni(){
                for m in 0..self.num_neuroni(){
                    self.internal_spike.get(n) += s.get(n) * self.intralayer_weights.get((n,m));
                }
            }

            return s
        }

        let spike_prec = self.prec_layer.aggiorna_Neuroni(ts, spike);
        let v = Vec::new();

        for n in 0..self.num_neuroni(){
            let mut tot = 0;

            for m in 0..self.prec_layer.num_neuroni(){
                tot = tot + spike_prec.get(m) * self.interlayer_weights.get((n,m)); // valutare se tali neuroni hanno generato uno spike
            }

            //decadenza internal_spike
            tot = tot + self.internal_spike.get(n);
            
            if tot > 1{
                tot=1;
            }

            v.append(self.neuroni.get(n).unwrap().potential_evolution(tot, ts)); //vettore di spike calcolati nel layer corrente  
               
        } 
        
        for n in 0..self.num_neuroni(){
            for m in 0..self.num_neuroni(){
                self.internal_spike.get(n) += v.get(n) * self.intralayer_weights.get((n,m));  //aggiornamento del valore pesato del layer corrente
            }
        }

        return v
    }

}