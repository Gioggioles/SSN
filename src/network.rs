use crate::layer;
use crate::{layer::Layer, lifNN::Neuron};
use ndarray::{Array2};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
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
                for l in 0..spike.len(){  
                    spike[l] = spike[l] + a[l];
                
            }

            let temp = Arc::new(Mutex::new(Vec::<f64>::new()));

            for m in 0..self.layers.get(i).unwrap().num_neuroni(){  //controlli tutti i neuroni
                let mut vt = Vec::new();
                let temp = temp.clone();
                let mut primo_layer = self.layers.get(i).unwrap().clone();
                let spike_temp = spike.clone();
                
                vt.push(std::thread::spawn(move || {
                    temp.lock().unwrap().push(primo_layer.get_neuroni_mut(m).unwrap().clone().potential_evolution(*spike_temp.get(m).unwrap(),ts));
                }));
             for v in vt{
                v.join().unwrap();
            }   
            
            }

            

            s = temp.lock().unwrap().to_vec();

            print!("Layer 1 =");
            for pollice in 0..s.len(){
                print!("{}", s.get(pollice).unwrap());
                print!("-");
            }
            print!("\n");


            let internal_temp = Arc::new(Mutex::new(Vec::from(self.layers[i].internal_spike.clone())));

            for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //Aggiornamento dei collegamenti intraLayer
                let mut vet_internal_spike =  Vec::new();
                let internal_temp = internal_temp.clone();
                let layer_temp = self.layers.get(i).unwrap().clone();
                let temporaneo = s.clone();
                vet_internal_spike.push(std::thread::spawn(move || {
                    for m in 0..layer_temp.num_neuroni(){                    
                        internal_temp.lock().unwrap()[m] += temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                    } 
              }));

              for v in vet_internal_spike{
                v.join().unwrap();
              }
        }
            self.layers[i].internal_spike = internal_temp.lock().unwrap().to_vec();

            }    
            else{      
                let temp = Arc::new(Mutex::new(Vec::<f64>::new()));

                for n in 0..self.layers.get(i).unwrap().num_neuroni(){
                    let mut layer_temp = self.layers.get(i).unwrap().clone();
                    let layer_temp_p = self.layers.get(i-1).unwrap().clone();
                    let mut vt = Vec::new();
                    let temp = temp.clone();
                    let temporaneo = s.clone();

                    vt.push(std::thread::spawn(move||{
                        let mut tot = 0.0;
                        for m in 0..layer_temp_p.num_neuroni(){

                            //let t = Arc::new(Mutex::new(Vec::<f64>::new())); IPOTESI DI THREAD ANCHE PER QUESTO for
                            //let mut vt = Vec::new();
                            tot = tot + temporaneo.get(m).unwrap() * layer_temp.get_interlayer_weight(n,m).unwrap(); // valutare se tali neuroni hanno generato uno spike
                        }
                    
                        tot = tot + *layer_temp.get_decadenza_internal_spike(ts).get(n).unwrap();

                        temp.lock().unwrap().push(layer_temp.get_neuroni_mut(n).unwrap().clone().potential_evolution(tot, ts)); //vettore di spike calcolati nel layer corrente    
                    }));

                    for v in vt{
                        v.join().unwrap();
                    }
                }

                s = temp.lock().unwrap().to_vec();

                let internal_temp = Arc::new(Mutex::new(Vec::from(self.layers[i].internal_spike.clone())));

                for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //Aggiornamento dei collegamenti intraLayer
                    let mut vet_internal_spike =  Vec::new();
                    let internal_temp = internal_temp.clone();
                    let layer_temp = self.layers.get(i).unwrap().clone();
                    let temporaneo = s.clone();
                    vet_internal_spike.push(std::thread::spawn(move || {
                        for m in 0..layer_temp.num_neuroni(){    //IPOTESI DI THREAD ANCHE PER QUESTO for                 
                            internal_temp.lock().unwrap()[m] += temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                        } 
                  }));
    
                  for v in vet_internal_spike{
                    v.join().unwrap();
                  }
            }

            for indicione in 0..s.len(){
                print!("{}", s.get(indicione).unwrap());
                print!("-");
            }
            print!("\n");

                self.layers[i].internal_spike = internal_temp.lock().unwrap().to_vec();
    
            }  
        }
        return s
    }       
}       