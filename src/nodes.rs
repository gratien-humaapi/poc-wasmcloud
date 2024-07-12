use std::{collections::HashMap, fmt::Debug};

use crate::wasi;

// Trait Node générique
pub trait Node {
    // Méthode execute qui prend un HashMap de paramètres
    fn execute(&self, params: HashMap<String, String>) -> Box<dyn std::fmt::Debug>;
}

impl Debug for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

// Struct AddNode qui implémente le trait Node pour T = Vec<i32>
pub struct TriggerNode;

impl Node for TriggerNode {
    fn execute(&self, _params: HashMap<String, String>) -> Box<dyn std::fmt::Debug> {
       Box::new(())
    }
}

pub struct  AddNode;

impl Node for AddNode {
    fn execute(&self, params: HashMap<String, String>) -> Box<dyn std::fmt::Debug> {
        let mut sum = 0;

        let value_str = params
            .get("value")
            .map_or_else(|| String::new(), |v| v.clone());

        // Récupérer le vecteur d'entiers à partir des paramètres
        if let Ok(value) = serde_json::from_str::<Vec<i32>>(value_str.as_str()) {
            // Calculer la somme des nombres dans le vecteur
            sum = value.iter().sum();
            Box::new(sum)
        } else {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Info,
                "",
                &format!("Error while converting value in AddNode"),
            );
            // Retourner une erreur ou une valeur par défaut si le paramètre n'est pas trouvé
            Box::new("Numbers to add not found")
        }
    }
}

// Struct Print qui implémente le trait Node pour T = String
pub struct PrintNode;

impl Node for PrintNode {
    fn execute(&self, params: HashMap<String, String>) -> Box<dyn std::fmt::Debug> {
        // Récupérer la chaîne à afficher à partir des paramètres
        let value_str = params
            .get("value")
            .map_or_else(|| String::new(), |v| v.clone());

        if let Ok(value) = serde_json::from_str::<String>(value_str.as_str()) {
            // Récupérer le vecteur d'entiers à partir des paramètres
            // Retourner le résultat sous forme de boîte de Debug pour l'affichage
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Info,
                "",
                &format!("Output : {value:?}"),
            );

            Box::new(())
        } else {
            wasi::logging::logging::log(
                wasi::logging::logging::Level::Info,
                "",
                &format!("Error while converting value in PrintNode"),
            );
            // Retourner une erreur ou une valeur par défaut si le paramètre n'est pas trouvé
            Box::new("Data not found")
        }
    }
}
