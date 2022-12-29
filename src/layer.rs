use ndarray::Array2;
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug)]

pub struct Layer<Neuron> {
    pub(crate) neuroni: Vec<Neuron>, // Lista di tutti i neuroni del layer.
    
    pub(crate) interlayer_weights: Array2<f64>, //Matrice degli interlayer, Per il primo layer deve essere una matrice diagonale.
    
    pub(crate) intralayer_weights: Array2<f64>, //Matrice quadrata dei pesi intra-layer (neuroni appartenti allo stesso layer).
    
    pub(crate) internal_spike: Vec<f64>, //Valore corrente dello spike per ogni Neuron di tale layer.

    pub(crate) ts_prec: f64, // Tempo dell'aggiornamento precedente.
}

impl<Neuron> Layer<Neuron> {
    pub fn new(neurons : Vec<Neuron>, intra_w : Array2<f64>, inter_w : Array2<f64>) -> Self{
        Self{
            internal_spike: vec![0.0; neurons.len()],  //vettore degli spike del layer (lista degli spike dei singoli neuroni)
            neuroni : neurons,             //vettore dei neuroni appartenenti al layer
            interlayer_weights : inter_w,  //matrice dei pesi inter-layer
            intralayer_weights : intra_w,  //matrice dei pesi intra-layer         
            ts_prec : 0.0                  //tempo dell'istante precedente
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
    pub fn get_decadenza_internal_spike(&mut self, t_s: f64) -> Vec<f64>{  //funzione di decadenza dell'internal spike, dovuta al trascorrere del tempo.
        let mut vt = Vec::new(); 
        let internal_temp = Arc::new(Mutex::new(Vec::from(self.internal_spike.clone())));//garantisco la mutua esclusione sul vettore degli spike
        let t_pr = self.ts_prec;
        for n in 0..self.internal_spike.len(){ 
            let internal_temp = internal_temp.clone();     
            vt.push(std::thread::spawn(move ||{
                let temp = internal_temp.lock().unwrap();
                let t = temp.get(n).unwrap() * ((t_pr-t_s)).exp(); //decadenza dello spike
                drop(temp);
                internal_temp.lock().unwrap()[n] = t;
            })); // creo un thread per ogni neurone che si occupa di eseguire la decadenza dell'internal spike del singolo neurone.
        }
        for v in vt{
            v.join().unwrap();
        } 

        self.internal_spike = internal_temp.lock().unwrap().to_vec();
        drop(internal_temp);
        self.ts_prec = t_s;

        return self.internal_spike.clone()
    }
}