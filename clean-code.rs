// clean-code.rust

// Exemplo de como não comentar seu código em Rust:
fn example_1() {
    // Iterar sobre o intervalo de 0 a 9
    // e invocar a função `do_something`
    // para cada iteração
    for i in 0..10 {
        do_something(i);
    }
}

// Seguindo o princípio "Documente o porquê, não o como", podemos melhorar:
fn example_2() {
    // Instanciar 10 threads para lidar com a carga de trabalho futura
    for i in 0..10 {
        do_something(i);
    }
}

// Isso explica o propósito, mas ainda não é ideal. Podemos expressar essa intenção diretamente no código:
fn example_3() {
    for worker_id in 0..10 {
        spawn_thread(worker_id);
    }
}

// Em Rust, podemos aproveitar as características da linguagem para tornar o código ainda mais expressivo:
fn example_4() {
    let num_workers = 10;
    (0..num_workers).for_each(|worker_id| {
        spawn_thread(worker_id);
    });
}

// Exemplo de leitura e configuração de arquivos:
fn main() {
    let config_path = std::env::args().nth(1).expect("Informe o caminho do arquivo de configuração");

    let config = configuration::parse(&config_path).expect("Falha ao analisar a configuração");
    
    // ...
}

// Função para analisar configurações com base na extensão do arquivo:
fn parse(filepath: &str) -> Result<Config, Box<dyn Error>> {
    match file_extension(filepath) {
        "json" => parse_json(filepath),
        "yaml" => parse_yaml(filepath),
        "toml" => parse_toml(filepath),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Extensão de arquivo desconhecida")))
    }
}

// Função auxiliar para obter a extensão do arquivo:
fn file_extension(filepath: &str) -> &str {
    filepath.rsplit('.').next().unwrap_or("")
}

// Função para imprimir marcas de cerveja:
fn print_brands_in_list(brands: &[BeerBrand]) {
    for b in brands { 
        println!("{}", b);
    }
}

// Função para converter uma lista de marcas de cerveja em uma lista de cervejas:
fn beer_brand_list_to_beer_list(beer_brands: &[BeerBrand]) -> Vec<Beer> {
    let mut beer_list = Vec::new();
    for brand in beer_brands {
        for beer in brand {
            beer_list.push(beer.clone());
        }
    }
    beer_list
}

// Exemplo de função para obter um item do banco de dados:
fn get_item(ctx: &Context, json: &[u8]) -> Result<Item, Box<dyn Error>> {
    let order = Item::from_json(json)?;
    
    if !get_user_from_context(ctx).is_admin() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Usuário não tem privilégios suficientes")));
    }
    
    db::get_item(order.item_id())
}

// Exemplo de funções para obter um item com base em uma referência:
fn get_item(extension: &str) -> Result<Item, Box<dyn Error>> {
    let reference = get_reference(extension)?;
    get_item_by_reference(&reference)
}

fn get_reference(extension: &str) -> Result<String, Box<dyn Error>> {
    db::reference_cache::get(extension)
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Referência não encontrada no cache")))
}

fn get_item_by_reference(reference: &str) -> Result<Item, Box<dyn Error>> {
    let item = get_item_from_cache(reference)?;
    if !item.is_active() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Item não está ativo")));
    }
    Ok(item)
}

fn get_item_from_cache(reference: &str) -> Result<Item, Box<dyn Error>> {
    db::item_cache::get(reference)
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Item não encontrado no cache")))
}

// Exemplo de criação de uma fila com opções configuráveis:
fn create_queue(name: &str, durable: bool, delete_on_exit: bool, exclusive: bool, no_wait: bool, arguments: Option<&[(&str, &str)]>) -> Result<(), Box<dyn std::error::Error>> {
    // Implementação da função
    Ok(())
}

// Exemplo de estrutura e função para criar uma fila com opções:
struct QueueOptions<'a> {
    name: &'a str,
    durable: bool,
    delete_on_exit: bool,
    exclusive: bool,
    no_wait: bool,
    arguments: Option<&'a [(&'a str, &'a str)]>,
}

fn create_queue(options: QueueOptions) -> Result<(), Box<dyn std::error::Error>> {
    // Implementação da função
    Ok(())
}

// Implementação do trait `Default` para `QueueOptions`:
impl Default for QueueOptions<'_> {
    fn default() -> Self {
        QueueOptions {
            name: "default",
            durable: false,
            delete_on_exit: false,
            exclusive: false,
            no_wait: false,
            arguments: None,
        }
    }
}

// Exemplo de função que retorna um resultado com base em um número:
fn do_complex() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Success".to_string())
}

fn main() {
    let mut val = String::new();
    let num = 32;

    match num {
        16 => {},
        32 => {
            let result = do_complex()?;
            val = result;
        },
        64 => {},
    }

    println!("{}", val);
}

// Exemplo de função que retorna uma string com base em um número:
fn get_string_result(num: i32) -> Result<String, Box<dyn std::error::Error>> {
    match num {
        16 => Ok(String::new()),
        32 => do_complex(),
        64 => Ok(String::new()),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid number"))),
    }
}

fn main() {
    let val = get_string_result(32)?;
    println!("{}", val);
}

// Exemplo de uso de canais em Rust:
fn main() {
    let items = get_items();
    let (sender, receiver) = std::sync::mpsc::channel();

    for item in items {
        // ...
    }
}

// Função para criar um sender e receber items:
fn create_sender() -> std::sync::mpsc::Sender<Item> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for item in receiver {
            // Processo item
        }
    });
    sender
}

fn main() {
    let sender = create_sender();
    // Use o sender
}

// Estrutura e implementação de `Sender` para enviar items:
struct Sender {
    sender: std::sync::mpsc::Sender<Item>,
}

impl Sender {
    fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for item in receiver {
                // Processo item
            }
        });
        Sender { sender }
    }

    fn send(&self, item: Item) {
        self.sender.send(item).expect("Falha ao enviar item");
    }
}

fn main() {
    let sender = Sender::new();
    // Use o sender
}

// Exemplo de tratamento de erros em Go:
package clean

import "errors"

var (
    NullItem = Item{}
    ErrItemNotFound = errors.New("item could not be found in the store")
)

func (store *Store) GetItem(id string) (Item, error) {
    store.mtx.Lock()
    defer store.mtx.Unlock()

    item, ok := store.items[id]
    if !ok {
        return NullItem, ErrItemNotFound
    }
    return item, nil
}

// Exemplo de tratamento de erros em Rust:
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum StoreError {
    ItemNotFound,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StoreError::ItemNotFound => write!(f, "Item could not be found in the store"),
        }
    }
}

pub struct Store {
    items: HashMap<String, Item>,
    // outros campos, se houver
}

impl Store {
    pub fn get_item(&self, id: &str) -> Result<Item, StoreError> {
        match self.items.get(id) {
            Some(item) => Ok(item.clone()),
            None => Err(StoreError::ItemNotFound),
        }
    }
}

// Exemplo de tratamento detalhado de erros em Rust:
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DetailedError {
    message: String,
}

impl fmt::Display for DetailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DetailedError {}

pub struct Store {
    items: HashMap<String, Item>,
    // outros campos, se houver
}

impl Store {
    pub fn get_item(&self, id: &str) -> Result<Item, DetailedError> {
        match self.items.get(id) {
            Some(item) => Ok(item.clone()),
            None => Err(DetailedError {
                message: format!("Item with ID '{}' not found in the store", id),
            }),
        }
    }
}
