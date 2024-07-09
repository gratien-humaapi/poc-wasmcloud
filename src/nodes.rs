use std::{collections::HashMap, fmt::Debug};

use serde_json::Value;

use crate::wasi;

// Trait Node générique
pub trait Node {
    // Méthode execute qui prend un HashMap de paramètres
    fn execute(&self, params: Option<HashMap<String, Value>>,input_data: Option<&Value>) -> Box<dyn std::fmt::Debug>;
}

impl Debug for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

// Struct AddNode qui implémente le trait Node pour T = Vec<i32>
pub struct AddNode;

impl Node for AddNode {
    fn execute(&self, params: Option<HashMap<String, Value>>, input_data: Option<&Value>) -> Box<dyn std::fmt::Debug> {
        let mut sum = 0;

        if let Some(x) = params {
            // Récupérer le vecteur d'entiers à partir des paramètres
            if let Some(numbers) = x.get("value") {
                if let Ok(numbers) = serde_json::from_value::<Vec<i32>>(numbers.clone()) {
                    // Calculer la somme des nombres dans le vecteur
                    sum = numbers.iter().sum();
                } else {
                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("Error while converting value in AddNode"),
                    );
                }
            }

            wasi::logging::logging::log(
                wasi::logging::logging::Level::Info,
                "",
                &format!("res = {}", sum),
            );
            Box::new(sum)
        } else {
            // Retourner une erreur ou une valeur par défaut si le paramètre n'est pas trouvé
            Box::new("Numbers to add not found")
        }
    }
}

// Struct Print qui implémente le trait Node pour T = String
pub struct PrintNode;

impl Node for PrintNode {
    fn execute(&self, params: Option<HashMap<String, Value>>, input_data: Option<&Value>) -> Box<dyn std::fmt::Debug> {
        // Récupérer la chaîne à afficher à partir des paramètres
        if let Some(x) = params {
            // Récupérer le vecteur d'entiers à partir des paramètres
            if let Some(value) = x.get("value") {
                if let Ok(res) = serde_json::from_value::<String>(value.clone()) {
                    // Retourner le résultat sous forme de boîte de Debug pour l'affichage
                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("Output : {res:?}"),
                    );
                } else {
                    wasi::logging::logging::log(
                        wasi::logging::logging::Level::Info,
                        "",
                        &format!("Error while converting value in PrintNode"),
                    );
                }
            }
            Box::new(())
        } else {
            // Retourner une erreur ou une valeur par défaut si le paramètre n'est pas trouvé
            Box::new("Data not found")
        }
    }
}
