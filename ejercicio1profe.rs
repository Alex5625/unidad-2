use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::OpenOptions;
use std::f32;
use std::io::Write;

fn create_blank_file(p: &Path) {
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn open_file_to_append(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}

// EL PRIMER CRASHEO ERA DE QUE NO IBA A EXISTIR SI SOLO ESTABA CREADO, PARA EXISTIR DEBIA TENER ESPACIOS EN BLANCOOOOO
fn open_file(p: &Path) -> (){
    create_blank_file(p);
    let _file = match File::open(&p){
        Err(_why) => panic!("El archivo no se puede abrir..."),
        Ok(file) => file,
    };
}

// fn open_file(p: &Path) -> (){

//     if Path::new(p).exists(){
//         let _file = match File::open(&p){
//             Err(_why) => panic!("El archivo no se puede abrir..."),
//             Ok(file) => file,
//         };
//     } else {
//         create_blank_file(p);
//         panic!("reinicie, porfavor")
//         //AQUI ESTA EL ERROR A CORREGIR (CRASHEO)
//     }
// }

// esta funcion puede ser metida a la api anterior
fn transformar_str_f32(string:&str) -> f32{
    let num: f32 = match string.parse::<f32>() {
        Err(_) => panic!("No es un numero compatible"),
        Ok(string) => string
    };
    return num
}
// esta tambien
fn imprimir_promedios(promedio: f32) -> bool{

    if promedio >= 4.0 {
        println!("Aprobo, con un promedio {}", promedio);
        return false;
    } else{
        println!("Reprobo, con un promedio {}", promedio);
        return true;
    }
}

fn todo_proceso(mut file:File) {

    if let Ok(lines) = read_lines("./notas.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(":");
                let mut suma:f32 = 0.0;
                let mut contador:u32 = 0;
                for s in split {
                    if contador == 0 {
                        //imprime lo de la posicion 0
                        println!("{}:", s);
                        // guarda en archivo REPORTE lo de la primera posicion (string)
                        file.write_all(s.as_bytes()).expect("tuvo un error!");
                    } else {
                        suma += transformar_str_f32(s);   
                    }
                    contador += 1;
                }
                let promedio:f32 = suma/6.0;
                if imprimir_promedios(promedio){
                    file.write_all(b" Reprobo\n").expect("tuvo un error!");
                } else {
                    file.write_all(b" Aprobo\n").expect("Tuvo un error!");
                }   
            }
        }
    }
    file.write_all(b"\n").expect("tuvo un eror!");
}

fn main() {
    //CREAR ARCHIVO
    let path = Path::new("reporte.txt");
    open_file(path);
    let file = open_file_to_append(path);
    todo_proceso(file);
}
