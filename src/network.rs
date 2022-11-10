use crate::{layer::{Layer, self}, lifNN::Neuron};
use ndarray::{Array2};

#[derive(Clone, Debug)]

pub struct Network<Layer> { //ricordare di aggiungere un altro ':' quando si crea la libreria

pub(crate) layers: Vec<Layer>,

pub(crate) num_layers : usize,
}

impl Network<Layer<Neuron>>{
    pub fn new(num_layers: usize, neurons: Vec<Vec<Neuron>>, interlayer_weights: Array2<f64>, intralayer_weights: Array2<f64>) -> Network<Layer<Neuron>> {
        let mut layers_new = Vec::new();
        for n in 0..num_layers{
           let l = Layer::new(*neurons.get(n).unwrap(), intralayer_weights, interlayer_weights);
           layers_new.push(l);
        }
        
        Network{
            layers : layers_new,
            num_layers : num_layers,
         } 
        
    }

    pub fn aggiorna_Neuroni (&mut self, ts : f64, spike : Vec<f64>) -> Vec<f64>{  //spike -> vettore di 0/1 dove ogni posizione corrisponde allo spike nel tempo ts del neurone nel layer precedente
        let mut s = Vec::new();
        
        for i in 0..self.num_layers{
            if(i==0){            
            //decadenza internal_spike
            spike = spike + self.layers.get(i).unwrap().internal_spike;
            

            for l in 0..spike.len(){  //controllo di spike, non può essere oltre 1
                if *spike.get(l).unwrap() > 1.0{
                    spike[l] = 1.0;
                }
            }

            for m in 0..self.layers.get(i).unwrap().num_neuroni(){  //controlli tutti i neuroni
                s.push(self.layers.get(i).unwrap().neuroni.get(m).unwrap().potential_evolution(*spike.get(m).unwrap(),ts));
            }

            for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //Aggiornamento dei collegamenti intraLayer
                for m in 0..self.layers.get(i).unwrap().num_neuroni(){
                    self.layers[i].internal_spike[n] += s.get(n).unwrap() * self.layers.get(i).unwrap().intralayer_weights.get((n,m)).unwrap();
                }
            }

        } 
        
        for n in 0..self.layers.get(i).unwrap().num_neuroni(){
            let mut tot = 0;

            for m in 0..self.layers.get(i-1).unwrap().num_neuroni(){
                tot = tot + s.get(m).unwrap() * self.layers.get(i).unwrap().interlayer_weights.get((n,m)).unwrap(); // valutare se tali neuroni hanno generato uno spike
            }

            //decadenza internal_spike
            tot = tot + *self.layers.get(i).unwrap().internal_spike.get(n).unwrap();
            
            if tot > 1{
                tot=1;
            }

            s = Vec::new();

            s.push(self.layers.get(i).unwrap().neuroni.get(n).unwrap().potential_evolution(tot, ts)); //vettore di spike calcolati nel layer corrente  
               
        }

        for n in 0..self.layers.get(i).unwrap().num_neuroni(){
            for m in 0..self.layers.get(i).unwrap().num_neuroni(){
                self.layers[i].internal_spike[n] += s.get(n).unwrap() * self.layers.get(i).unwrap().intralayer_weights.get((n,m)).unwrap();  //aggiornamento del valore pesato del layer corrente
            }
        }  
     }
     return s
    }       
}       



        
        /*if (self.prec_layer == null){ //primo layer
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
    }*/
    