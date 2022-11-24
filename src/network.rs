
use crate::{layer::Layer, lifNN::Neuron};
use ndarray::{Array2};

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
        let mut s = Vec::new();
        
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

            for l in 0..spike.len(){  //controllo di spike, non può essere oltre 1
                if *spike.get(l).unwrap() > 1.0{
                    spike[l] = 1.0;
                }
            }


            for m in 0..self.layers.get(i).unwrap().num_neuroni(){  //controlli tutti i neuroni
                s.push(self.layers.get(i).unwrap().neuroni.get(m).unwrap().clone().potential_evolution(*spike.get(m).unwrap(),ts));
            }


            for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //Aggiornamento dei collegamenti intraLayer
                for m in 0..self.layers.get(i).unwrap().num_neuroni(){
                    self.layers[i].internal_spike[n] += s.get(n).unwrap() * self.layers.get(i).unwrap().intralayer_weights.get((n,m)).unwrap();
                }
            }

        } 
        else{

        for n in 0..self.layers.get(i).unwrap().num_neuroni(){
            let mut tot = 0.0;
            print!("{}", self.layers.get(i).unwrap().num_neuroni());
            for m in 0..self.layers.get(i-1).unwrap().num_neuroni(){
                tot = tot + s.get(m).unwrap() * self.layers.get(i).unwrap().interlayer_weights.get((n,m)).unwrap(); // valutare se tali neuroni hanno generato uno spike
            } //errore nel s.unwrap()

            
            tot = tot + *self.layers.get(i).unwrap().clone().get_decadenza_internal_spike(ts).get(n).unwrap();
            
            if tot > 1.0{
                tot=1.0;
            }

            s = Vec::new(); //PERDO I DATI

            s.push(self.layers.get(i).unwrap().neuroni.get(n).unwrap().clone().potential_evolution(tot, ts)); //vettore di spike calcolati nel layer corrente  
               
        }

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