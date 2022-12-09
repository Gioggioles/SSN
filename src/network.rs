use crate::{layer::Layer, lifNN::Neuron};
use ndarray::{Array2};
use std::thread;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Network<Layer> { //ricordare di aggiungere un altro ':' quando si crea la libreria
    pub(crate) layers: Vec<Layer>,

    pub(crate) num_layers : usize,
}
impl Network<Layer<Neuron>>{

    pub fn new() -> Self{
        Network { layers: Vec::new(), num_layers: 0 }
    }

    pub fn add_layer(&mut self, neurons: Vec<Neuron>, interlayer_weights: Array2<f64>, intralayer_weights: Array2<f64>) { 
           let l = Layer::new(neurons.clone(), intralayer_weights.clone(), interlayer_weights.clone());
           self.num_layers += 1;
           self.layers.push(l);        //da rivalutare
    }

    pub fn aggiorna_neuroni (&mut self, ts : f64, mut spike : Vec<f64>) -> Vec<f64>{  //spike -> vettore di 0/1 dove ogni posizione corrisponde allo spike del neurone i-esimo originale
        let mut s = Vec::new(); //pesi del layer precedente
        
        for i in 0..self.num_layers{
            if i==0 {                
            
            //spike = spike + self.layers.get(i).unwrap().get_decadenza_internal_spike(ts);
            let mut m = self.layers.get(i).unwrap().clone();
            let a = m.get_decadenza_internal_spike(ts);
            if a.len() == spike.len(){
                for l in 0..spike.len(){  
                    spike[l] = spike[l] + a[l];
                }
            }


            for m in 0..self.layers.get(i).unwrap().num_neuroni(){  //controlli tutti i neuroni
                s.push(self.layers.get(i).unwrap().neuroni.get(m).unwrap().clone().potential_evolution(*spike.get(m).unwrap(),ts));
            }
            print!("Layer 1 =");
            for pollice in 0..s.len(){
                print!("{}", s.get(pollice).unwrap());
                print!("-");
            }
            print!("\n");

            for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //Aggiornamento dei collegamenti intraLayer
                for m in 0..self.layers.get(i).unwrap().num_neuroni(){
                    self.layers[i].internal_spike[m] += s.get(n).unwrap() * self.layers.get(i).unwrap().intralayer_weights.get((n,m)).unwrap();
                } //[[0.0, 0.5, 0.2], [1.0, 0.0, 0.7], [0.1, 0.6, 0.0]]  -  [[0.0, 0.9],[0.8, 0.0]]   -  [[0.0, 0.7, 0.3],[1.1, 0.0, 0.8],[0.3, 0.5, 0.0]]
            }

        } 
        else{      
        for n in 0..self.layers.get(i).unwrap().num_neuroni(){
            let mut layer_temp = self.layers.get(i).unwrap().clone();
            let layer_temp_p = self.layers.get(i-1).unwrap().clone();
            let mut temp = Vec::new();
            let mut vt = Vec::new();
    
            vt.push(std::thread::spawn(move||{
            let mut tot = 0.0;
            for m in 0..layer_temp_p.num_neuroni(){
                tot = tot + s.get(m).unwrap() * layer_temp.get_interlayer_weight(n,m).unwrap(); // valutare se tali neuroni hanno generato uno spike
            }
                //[[0.9, 1.1, 1.1], [0.7, 0.65, 1.0]]  -   [[0.8, 0.8], [0.9, 0.7], [0.7, 1.0]]
        
            tot = tot + *layer_temp.get_decadenza_internal_spike(ts).get(n).unwrap();

            //inseriscinelvettore struct -> vettore mutex,condvar questa struct sara messa dentro un ARC

            temp.push(layer_temp.get_neuroni_mut(n).unwrap().clone().potential_evolution(tot, ts)); //vettore di spike calcolati nel layer corrente    
        }));
        for v in vt{
            v.join().unwrap();
        }
    }

        s = temp.clone();

        for indicione in 0..s.len(){
            print!("{}", s.get(indicione).unwrap());
            print!("-");
        }
        print!("\n");
        
        for n in 0..self.layers.get(i).unwrap().num_neuroni(){
            for m in 0..self.layers.get(i).unwrap().num_neuroni(){
                self.layers[i].internal_spike[n] += s.get(n).unwrap() * self.layers.get(i).unwrap().intralayer_weights.get((n,m)).unwrap();  //aggiornamento del valore pesato del layer corrente
            }
        }
        }  
     }
     return s
    }       
}       