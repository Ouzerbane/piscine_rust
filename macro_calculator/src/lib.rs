// Food {
//     name: <name>,
//     calories: (<calories_in_kJ>kJ, <calories_in_kcal>kcal),
//     fats: <fats_in_grams>,
//     carbs: <carbs_in_grams>,
//     proteins: <proteins_in_grams>,
//     nbr_of_portions: <portions>
// }
use json::JsonValue ;

pub struct Food {
    // expected public fields
   pub name:String ,
    pub calories : (String , String) ,
    pub proteins : f64 ,
    pub fats : f64 ,
    pub carbs : f64 ,
    pub nbr_of_portions:f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
   
    let mut  data = JsonValue::new_object();
    
     if foods.len() == 0 {
         data["cals"] = (0.0).into() ;
         data["carbs"] = (0.0).into();
         data["proteins"] = (0.0).into();
         data["fats"] = (0.0).into();
        return data ;   
    }
    for i in foods {
         let clal = i.calories.1.replace("kcal","");
        let add = clal.parse::<f64>().unwrap();
        data["cals"] = (round2(data["cals"].as_f64().unwrap_or(0.0)+add * i.nbr_of_portions)).into();

        data["carbs"] = (round2(data["carbs"].as_f64().unwrap_or(0.0) + i.carbs * i.nbr_of_portions)).into() ;
        data["proteins"] = (round2(data["proteins"].as_f64().unwrap_or(0.0)+i.proteins * i.nbr_of_portions)).into();
        data["fats"] = (round2(data["fats"].as_f64().unwrap_or(0.0)+i.fats * i.nbr_of_portions)).into();
       
    }
    data
    // todo!()
}

fn round2(value: f64) -> f64 {
    (value*100.0).round()/100.0
}