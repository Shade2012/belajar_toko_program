    use std::{io, usize};

    #[derive(Debug)]
    struct Barang {
        name:String,
        price:u64
    }
    fn main() {
        //User Variable
        let is_continue : bool = true;
        let mut total: u64 = 0;
        let mut _cart: Vec<&Barang> = vec![];

        // Others Variable
        let mut counter =0;
        let chitato: Barang = Barang { 
            name: String::from("Chitato"),
            price: 5000 
        };
        let apel: Barang = Barang { 
            name: String::from("Apel"),
            price: 3000 
        };
        let all_barang: Vec<&Barang>  = vec![&apel,&chitato];

        //Input Variable
        let mut input_barang = String::new();
        let mut input_jumlah_barang = String::new();
        let mut input = String::new();
        let mut input_uang = String::new();
        let mut input_is_continue: String = String::new();

        //Start Program
        println!("Siapa Nama mu: ");
        io::stdin().read_line(&mut input).expect("Failed to read line"); 
        println!("Selamat Datang di Toko Kali {},\n mau beli apa disini ?",input.trim());
        while is_continue {
        while &counter < &all_barang.len()  {
            let barang = &all_barang[counter];
            println!("{}. {} {} Rp",&counter+1,&barang.name,&barang.price);        
            counter += 1;
        }
        counter = 1;
        

        println!("Silahkan Input Nomor barang yang ingin anda beli");
        io::stdin().read_line(&mut input_barang).expect("Failed to read line");
        println!("Anda Ingin beli berapa ?");
        io::stdin().read_line(&mut input_jumlah_barang).expect("Failed to read line");
        match &input_barang.trim().parse::<usize>() {
            Ok(parsed_number) => {
                let choice = parsed_number -1;
            let barang_selected :&Barang = all_barang[choice];
            match &input_jumlah_barang.trim().parse::<usize>() {
                Ok(parsed_number) => {
                    let jumlah = parsed_number;
                    while &counter <= jumlah {
                        counter += 1;
                        _cart.push(barang_selected);
                    }
                }
                Err(_) => {
                    println!("Error: Input is not a valid usize di jumlah");
                }
            }
            }
            Err(_) => {
                println!("Error: Input is not a valid usize di pilih barang");
            }
        }
        println!("Apakah anda ingin berbelanja lagi ? \nIya Tidak");
        io::stdin().read_line(&mut input_is_continue).expect("Failed to read line");
        let input_is_continue = input_is_continue.trim().to_lowercase(); // Convert to lowercase and trim whitespace

        
        if input_is_continue == "tidak" {
            break;
        } else if input_is_continue != "iya" {
            break;
        }
        counter = 0;
        input_barang.clear();        
        input_jumlah_barang.clear();
        }


        //End Section Almost Finished
        counter = 0;
        println!("ini isi keranjang anda");
        while counter < _cart.len()  {
            let barang = &_cart[counter];
            println!("{}. {} {} Rp",&counter+1,&barang.name,&barang.price);      
            total += barang.price; 
            counter += 1;
        }

        println!("Silahkan Bayar,\nini total mu {}",&total);

        
        println!("Berapa uang mu?");
        io::stdin().read_line(&mut input_uang).expect("Failed to read line");
        let input_uang = input_uang.trim();
        match input_uang.parse::<u64>() {
            Ok(parsed_number) => {
            let uang = parsed_number;
        if &uang > &total {
            let kembalian = &uang -&total;
            println!("Ini Kembalian mu {}",kembalian);
        }else if &uang == &total {
            println!("Uang mu pas");
        }else{
            println!("Maaf Uang mu kurang");
        }
        println!("Selamat Datang Kembali");
            }
            Err(_) => {
            println!("Error: Input is not a valid usize di jumlah");
        }
        }
    }




