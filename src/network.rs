use crate::{layer::Layer, lif::Neuron};
use ndarray::Array2;
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Network<Layer> { 
    pub(crate) layers: Vec<Layer>, //Vettore con tutti i layer della rete

    pub(crate) num_layers : usize, //Numero di layers
}

impl Network<Layer<Neuron>>{
    pub fn new() -> Self{
        Network { layers: Vec::new(), num_layers: 0 }
    }

    pub fn add_layer(&mut self, neurons: Vec<Neuron>, interlayer_weights: Array2<f64>, intralayer_weights: Array2<f64>) { 
        let l = Layer::new(neurons.clone(), intralayer_weights.clone(), interlayer_weights.clone());
        self.num_layers += 1;
        self.layers.push(l);
    }

    pub fn aggiorna_neuroni (&mut self, ts : f64, mut spike : Vec<f64>) -> Vec<f64>{
        let mut s = Vec::new();
        
        for i in 0..self.num_layers{
            if i==0 {                
                let mut m = self.layers.get(i).unwrap().clone();
                let a = m.get_decadenza_internal_spike(ts);
                for l in 0..spike.len(){  
                    spike[l] = spike[l] + a[l];
                }

                let temp = Arc::new(Mutex::new(Vec::<f64>::new()));
                let mut vt = Vec::new();
                for m in 0..self.layers.get(i).unwrap().num_neuroni(){ 
                    let temp = temp.clone();
                    let mut primo_layer = self.layers.get(i).unwrap().clone();
                    let spike_temp = spike.clone();
                    
                    vt.push(std::thread::spawn(move || {
                        temp.lock().unwrap().push(primo_layer.get_neuroni_mut(m).unwrap().clone().potential_evolution(*spike_temp.get(m).unwrap(),ts));
                    })); //crea un thread per ogni Neuron del layer, aggiorna il potenziale di membrana e ritorna lo spike del Neuron 
                }
                for v in vt{
                    v.join().unwrap();
                }

                s = temp.lock().unwrap().to_vec();

                print!("Layer 1 =");
                for ciclo in 0..s.len(){
                    print!("{}", s.get(ciclo).unwrap());
                    print!("-");
                }
                print!("\n");


                let internal_temp = Arc::new(Mutex::new(Vec::from(self.layers[i].internal_spike.clone()))); // Inserisco l'internal_spike dentro un puntatore di tipo Arc per utilizzarlo nel thread
                let mut vet_internal_spike =  Vec::new();

                for n in 0..self.layers.get(i).unwrap().num_neuroni(){ 
                    let internal_temp = internal_temp.clone();
                    let layer_temp = self.layers.get(i).unwrap().clone();
                    let temporaneo = s.clone();
                    vet_internal_spike.push(std::thread::spawn(move || {
                        for m in 0..layer_temp.num_neuroni(){                    
                            internal_temp.lock().unwrap()[m] += temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                        }  //creo un thread per ogni Neuron che aggiorna il valore dell'internal_spike, aggiungendo i nuovi spike degli altri nueroni del proprio layer, pesati secondo la matrice di intrlayer_weights
                    }));
                }
                for v in vet_internal_spike{
                    v.join().unwrap();
                }

                self.layers[i].internal_spike = internal_temp.lock().unwrap().to_vec();  // Inserisco il risultato dei thread nel layer

            }    
            else{      
                let temp = Arc::new(Mutex::new(Vec::<f64>::new()));
                let mut vt = Vec::new();
                for n in 0..self.layers.get(i).unwrap().num_neuroni(){
                    let mut layer_temp = self.layers.get(i).unwrap().clone();
                    let layer_temp_p = self.layers.get(i-1).unwrap().clone();
                    let temp = temp.clone();
                    let temporaneo = s.clone();

                    vt.push(std::thread::spawn(move||{
                        let mut tot = 0.0;
                        for m in 0..layer_temp_p.num_neuroni(){
                            tot = tot + temporaneo.get(m).unwrap() * layer_temp.get_interlayer_weight(n,m).unwrap();
                        }
                        tot = tot + *layer_temp.get_decadenza_internal_spike(ts).get(n).unwrap();
                        temp.lock().unwrap().push(layer_temp.get_neuroni_mut(n).unwrap().clone().potential_evolution(tot, ts));    
                    })); //creo un thread per ogni neurone che, calcola la weighted_sum del neurone, aggiorna il potenziale di membrana e ritorna lo spike del Neuron
                }
                for v in vt{
                    v.join().unwrap();
                }

                s = temp.lock().unwrap().to_vec();

                let internal_temp = Arc::new(Mutex::new(Vec::from(self.layers[i].internal_spike.clone())));
                let mut vet_internal_spike =  Vec::new();
                for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //Aggiornamento dell'internal_spike dovuto ai collegamenti intralayer (THREAD)
                    let internal_temp = internal_temp.clone();
                    let layer_temp = self.layers.get(i).unwrap().clone();
                    let temporaneo = s.clone();
                    vet_internal_spike.push(std::thread::spawn(move || {
                        for m in 0..layer_temp.num_neuroni(){                  
                            internal_temp.lock().unwrap()[m] += temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                        } 
                    }));
                    
                }
                for v in vet_internal_spike{
                        v.join().unwrap();
                }

                for indice in 0..s.len(){
                    print!("{}", s.get(indice).unwrap());
                    print!("-");
                }
                print!("\n");
                self.layers[i].internal_spike = internal_temp.lock().unwrap().to_vec();
            }  
        }
        return s
    }       
}       