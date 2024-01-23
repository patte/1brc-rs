# 1BRC in rust

[The One Billion Row Challenge](https://github.com/gunnarmorling/1brc).

This solves the 1BRCthe in rust with [polars](https://docs.pola.rs/). Most of the time was spent thinking to go crazy until the missing feature flag was discovered.

## results
Machine: MacBook Air (M2, 2023)

### stable
Rust version: 1.73.0-aarch64-apple-darwin

Takes ~400s to complete and used more than 40GB of RAM, which means it was swapping a lot.

### nightly
Rust version: nightly

Takes ~400s.

## Output (stable)
```bash
cargo build --release && time target/release/one_billion_row_challenge_polars
   Compiling one_billion_row_challenge_polars v0.1.0 (/Users/patte/src/1BRC-rs/polars)
    Finished release [optimized] target(s) in 1m 36s
num stations: 413
Abha;-33.9,18.0,67.5
Abidjan;-24.1,26.0,74.6
Abéché;-18.9,29.4,78.1
Accra;-21.4,26.4,76.0
Addis Ababa;-32.8,16.0,66.4
Adelaide;-32.8,17.3,70.0
Aden;-19.4,29.1,80.8
Ahvaz;-23.9,25.4,73.2
Albuquerque;-36.7,14.0,66.9
Alexandra;-36.0,11.0,61.2
Alexandria;-31.1,20.0,75.7
Algiers;-31.0,18.2,66.2
Alice Springs;-31.2,21.0,71.1
Almaty;-39.0,10.0,61.9
Amsterdam;-39.3,10.2,62.4
Anadyr;-56.5,-6.9,41.0
Anchorage;-46.9,2.8,53.9
Andorra la Vella;-44.1,9.8,56.9
Ankara;-35.6,12.0,59.1
Antananarivo;-34.2,17.9,70.0
Antsiranana;-23.3,25.2,77.2
Arkhangelsk;-48.3,1.3,49.5
Ashgabat;-31.0,17.1,67.6
Asmara;-33.7,15.6,63.3
Assab;-21.1,30.5,80.7
Astana;-51.5,3.5,54.0
Athens;-30.0,19.2,69.2
Atlanta;-31.5,17.0,67.3
Auckland;-33.2,15.2,63.9
Austin;-29.9,20.7,73.0
Baghdad;-28.2,22.8,73.1
Baguio;-28.2,19.5,69.8
Baku;-35.9,15.1,65.6
Baltimore;-38.7,13.1,64.7
Bamako;-21.3,27.8,81.7
Bangkok;-20.6,28.6,79.7
Bangui;-26.1,26.0,74.2
Banjul;-23.9,26.0,79.4
Barcelona;-28.7,18.2,72.0
Bata;-23.9,25.1,77.1
Batumi;-38.9,14.0,67.2
Beijing;-35.7,12.9,62.8
Beirut;-28.4,20.9,69.6
Belgrade;-39.4,12.5,59.8
Belize City;-27.0,26.7,75.3
Benghazi;-28.4,19.9,69.1
Bergen;-42.3,7.7,57.5
Berlin;-40.4,10.3,60.1
Bilbao;-36.2,14.7,66.5
Birao;-28.0,26.5,76.1
Bishkek;-37.0,11.3,61.7
Bissau;-27.0,27.0,74.2
Blantyre;-25.1,22.2,72.1
Bloemfontein;-37.4,15.6,64.4
Boise;-35.9,11.4,59.9
Bordeaux;-38.5,14.2,63.2
Bosaso;-20.4,30.0,79.5
Boston;-37.4,10.9,59.4
Bouaké;-23.0,26.0,75.3
Bratislava;-45.4,10.5,62.3
Brazzaville;-28.2,25.0,75.1
Bridgetown;-27.4,27.0,77.5
Brisbane;-27.7,21.4,70.4
Brussels;-41.0,10.5,61.8
Bucharest;-36.9,10.8,59.3
Budapest;-36.8,11.3,60.7
Bujumbura;-26.0,23.8,73.3
Bulawayo;-33.5,18.9,69.5
Burnie;-36.8,13.1,63.9
Busan;-37.1,15.0,64.4
Cabo San Lucas;-26.3,23.9,73.9
Cairns;-30.4,25.0,79.9
Cairo;-27.1,21.4,70.8
Calgary;-47.6,4.4,53.6
Canberra;-38.6,13.1,63.1
Cape Town;-32.4,16.2,67.3
Changsha;-31.1,17.4,70.5
Charlotte;-34.4,16.1,70.1
Chiang Mai;-28.1,25.8,74.5
Chicago;-38.8,9.8,60.8
Chihuahua;-32.3,18.6,65.6
Chittagong;-23.3,25.9,75.9
Chișinău;-44.9,10.2,59.1
Chongqing;-30.4,18.6,69.0
Christchurch;-36.0,12.2,63.2
City of San Marino;-35.7,11.8,65.2
Colombo;-20.3,27.4,78.0
Columbus;-44.2,11.7,59.4
Conakry;-22.2,26.4,82.8
Copenhagen;-39.9,9.1,56.2
Cotonou;-21.0,27.2,81.2
Cracow;-37.4,9.3,63.0
Da Lat;-32.4,17.9,66.0
Da Nang;-23.9,25.8,74.6
Dakar;-26.2,24.0,73.2
Dallas;-30.1,19.0,70.1
Damascus;-31.5,17.0,65.3
Dampier;-22.6,26.4,77.5
Dar es Salaam;-24.4,25.8,74.7
Darwin;-22.6,27.6,77.7
Denpasar;-28.0,23.7,73.4
Denver;-40.4,10.4,59.0
Detroit;-37.9,10.0,58.8
Dhaka;-26.9,25.9,78.5
Dikson;-63.9,-11.1,37.9
Dili;-26.2,26.6,76.1
Djibouti;-19.2,29.9,79.0
Dodoma;-25.1,22.7,80.2
Dolisie;-25.0,24.0,77.6
Douala;-26.1,26.7,74.0
Dubai;-24.0,26.9,76.7
Dublin;-45.5,9.8,56.9
Dunedin;-39.8,11.1,59.5
Durban;-29.6,20.6,71.5
Dushanbe;-36.0,14.7,66.4
Edinburgh;-41.8,9.3,58.8
Edmonton;-46.1,4.2,59.8
El Paso;-31.0,18.1,68.6
Entebbe;-27.7,21.0,72.3
Erbil;-30.3,19.5,78.4
Erzurum;-43.4,5.1,58.9
Fairbanks;-50.9,-2.3,50.3
Fianarantsoa;-35.3,17.9,69.1
Flores,  Petén;-21.9,26.4,77.1
Frankfurt;-44.3,10.6,68.4
Fresno;-30.8,17.9,65.9
Fukuoka;-32.0,17.0,66.0
Gaborone;-28.7,21.0,72.6
Gabès;-32.1,19.5,70.1
Gagnoa;-22.4,26.0,74.3
Gangtok;-35.2,15.2,68.3
Garissa;-23.7,29.3,82.4
Garoua;-20.5,28.3,79.3
George Town;-21.4,27.9,80.0
Ghanzi;-28.4,21.4,69.8
Gjoa Haven;-68.7,-14.4,34.2
Guadalajara;-31.6,20.9,70.5
Guangzhou;-31.4,22.4,71.1
Guatemala City;-30.8,20.4,73.0
Halifax;-41.0,7.5,58.5
Hamburg;-45.7,9.7,60.1
Hamilton;-35.4,13.8,61.8
Hanga Roa;-27.6,20.5,67.9
Hanoi;-28.1,23.6,76.3
Harare;-38.9,18.4,69.3
Harbin;-46.4,5.0,57.5
Hargeisa;-30.8,21.7,67.5
Hat Yai;-22.4,27.0,76.8
Havana;-25.9,25.2,72.8
Helsinki;-45.9,5.9,52.1
Heraklion;-29.9,18.9,68.2
Hiroshima;-30.8,16.3,64.9
Ho Chi Minh City;-20.8,27.4,75.9
Hobart;-33.4,12.7,62.6
Hong Kong;-25.3,23.3,74.4
Honiara;-23.1,26.5,74.9
Honolulu;-22.9,25.4,74.2
Houston;-26.9,20.8,74.2
Ifrane;-42.2,11.4,61.2
Indianapolis;-35.5,11.8,61.4
Iqaluit;-59.9,-9.3,39.2
Irkutsk;-46.7,1.0,52.7
Istanbul;-33.1,13.9,63.7
Jacksonville;-26.5,20.3,71.9
Jakarta;-22.6,26.7,74.5
Jayapura;-19.0,27.0,75.6
Jerusalem;-32.7,18.3,70.3
Johannesburg;-35.1,15.5,63.8
Jos;-26.4,22.8,72.2
Juba;-22.8,27.8,80.2
Kabul;-44.5,12.1,61.9
Kampala;-27.5,20.0,69.7
Kandi;-22.7,27.7,80.3
Kankan;-22.0,26.5,76.0
Kano;-23.2,26.4,77.3
Kansas City;-36.6,12.5,61.5
Karachi;-20.8,26.0,74.1
Karonga;-23.9,24.4,71.6
Kathmandu;-30.3,18.3,65.1
Khartoum;-20.2,29.9,78.3
Kingston;-23.7,27.4,78.7
Kinshasa;-28.6,25.3,76.6
Kolkata;-26.5,26.7,78.8
Kuala Lumpur;-22.9,27.3,81.2
Kumasi;-24.7,26.0,73.5
Kunming;-33.8,15.7,67.5
Kuopio;-50.1,3.4,50.9
Kuwait City;-24.2,25.7,73.9
Kyiv;-43.2,8.4,59.4
Kyoto;-34.2,15.8,70.1
La Ceiba;-20.6,26.2,74.8
La Paz;-24.0,23.7,73.2
Lagos;-24.0,26.8,75.9
Lahore;-23.5,24.3,75.5
Lake Havasu City;-25.1,23.7,71.7
Lake Tekapo;-40.3,8.7,55.7
Las Palmas de Gran Canaria;-30.1,21.2,70.2
Las Vegas;-28.6,20.3,70.1
Launceston;-36.1,13.1,66.8
Lhasa;-40.6,7.6,57.1
Libreville;-27.2,25.9,78.6
Lisbon;-33.1,17.5,67.1
Livingstone;-27.9,21.8,76.4
Ljubljana;-38.6,10.9,60.8
Lodwar;-23.2,29.3,79.3
Lomé;-23.2,26.9,73.3
London;-41.2,11.3,62.0
Los Angeles;-34.4,18.6,65.4
Louisville;-35.9,13.9,61.8
Luanda;-27.4,25.8,76.5
Lubumbashi;-29.1,20.8,71.9
Lusaka;-28.2,19.9,77.8
Luxembourg City;-38.7,9.3,63.6
Lviv;-41.3,7.8,61.7
Lyon;-40.3,12.5,62.9
Madrid;-34.6,15.0,65.4
Mahajanga;-23.3,26.3,78.6
Makassar;-25.9,26.7,74.7
Makurdi;-24.8,26.0,77.6
Malabo;-26.5,26.3,78.3
Malé;-21.2,28.0,76.1
Managua;-21.7,27.3,79.8
Manama;-26.5,26.5,74.7
Mandalay;-18.6,28.0,78.6
Mango;-19.9,28.1,75.6
Manila;-20.1,28.4,77.4
Maputo;-27.4,22.8,71.6
Marrakesh;-30.3,19.6,68.3
Marseille;-35.3,15.8,72.7
Maun;-28.4,22.4,71.6
Medan;-25.9,26.5,79.1
Mek'ele;-28.7,22.7,70.1
Melbourne;-30.9,15.1,63.8
Memphis;-31.1,17.2,69.2
Mexicali;-23.5,23.1,73.5
Mexico City;-30.9,17.5,69.5
Miami;-24.3,24.9,72.3
Milan;-35.9,13.0,61.6
Milwaukee;-41.3,8.9,61.0
Minneapolis;-40.8,7.8,57.1
Minsk;-41.5,6.7,54.1
Mogadishu;-22.4,27.1,75.9
Mombasa;-25.7,26.3,78.2
Monaco;-33.2,16.4,64.4
Moncton;-49.7,6.1,55.1
Monterrey;-27.4,22.3,71.8
Montreal;-44.7,6.8,59.8
Moscow;-44.3,5.8,58.8
Mumbai;-25.2,27.1,72.6
Murmansk;-51.0,0.6,54.4
Muscat;-23.5,28.0,76.0
Mzuzu;-32.7,17.7,70.4
N'Djamena;-23.8,28.3,77.7
Naha;-25.2,23.1,72.7
Nairobi;-28.1,17.8,69.4
Nakhon Ratchasima;-22.2,27.3,86.0
Napier;-35.3,14.6,66.7
Napoli;-32.2,15.9,62.5
Nashville;-33.4,15.4,68.3
Nassau;-25.0,24.6,73.4
Ndola;-30.7,20.3,68.8
New Delhi;-24.0,25.0,72.3
New Orleans;-32.6,20.7,68.5
New York City;-35.2,12.9,64.9
Ngaoundéré;-25.2,22.0,71.2
Niamey;-24.8,29.3,77.5
Nicosia;-29.6,19.7,71.0
Niigata;-38.0,13.9,60.7
Nouadhibou;-28.8,21.3,73.2
Nouakchott;-24.4,25.7,76.3
Novosibirsk;-46.9,1.7,51.0
Nuuk;-52.9,-1.4,51.9
Odesa;-40.7,10.7,57.2
Odienné;-23.6,26.0,77.1
Oklahoma City;-35.0,15.9,68.5
Omaha;-40.8,10.6,59.3
Oranjestad;-22.9,28.1,76.4
Oslo;-46.5,5.7,55.4
Ottawa;-42.3,6.6,60.2
Ouagadougou;-27.3,28.3,80.9
Ouahigouya;-20.4,28.6,76.1
Ouarzazate;-31.5,18.9,69.4
Oulu;-48.0,2.7,50.9
Palembang;-24.9,27.3,81.6
Palermo;-30.9,18.5,68.8
Palm Springs;-26.8,24.5,78.7
Palmerston North;-35.6,13.2,62.0
Panama City;-22.4,28.0,78.7
Parakou;-25.4,26.8,78.4
Paris;-37.3,12.3,63.1
Perth;-29.2,18.7,66.4
Petropavlovsk-Kamchatsky;-47.6,1.9,53.0
Philadelphia;-36.5,13.2,65.8
Phnom Penh;-23.2,28.3,83.2
Phoenix;-24.0,23.9,75.8
Pittsburgh;-40.6,10.8,61.3
Podgorica;-37.5,15.3,64.2
Pointe-Noire;-24.6,26.1,74.7
Pontianak;-20.5,27.7,76.2
Port Moresby;-20.5,26.9,76.4
Port Sudan;-21.1,28.4,78.5
Port Vila;-27.0,24.3,77.4
Port-Gentil;-29.9,26.0,73.6
Portland (OR);-36.7,12.4,61.9
Porto;-32.7,15.7,65.7
Prague;-41.6,8.4,62.7
Praia;-26.6,24.4,77.3
Pretoria;-35.2,18.2,68.2
Pyongyang;-44.9,10.8,65.6
Rabat;-38.5,17.2,69.8
Rangpur;-23.4,24.4,74.2
Reggane;-19.8,28.3,77.3
Reykjavík;-50.2,4.3,57.2
Riga;-44.5,6.2,52.6
Riyadh;-24.6,26.0,79.2
Rome;-31.8,15.2,71.6
Roseau;-22.9,26.2,74.8
Rostov-on-Don;-39.3,9.9,58.4
Sacramento;-32.5,16.3,66.0
Saint Petersburg;-44.8,5.8,56.7
Saint-Pierre;-45.9,5.7,62.0
Salt Lake City;-40.9,11.6,57.7
San Antonio;-28.9,20.8,70.5
San Diego;-31.0,17.8,66.7
San Francisco;-39.4,14.6,65.7
San Jose;-39.0,16.4,68.2
San José;-27.0,22.6,74.6
San Juan;-23.6,27.2,82.0
San Salvador;-24.9,23.1,70.1
Sana'a;-30.6,20.0,70.9
Santo Domingo;-23.2,25.9,73.0
Sapporo;-41.2,8.9,56.8
Sarajevo;-37.3,10.1,59.4
Saskatoon;-46.8,3.3,54.0
Seattle;-40.5,11.3,60.9
Seoul;-37.4,12.5,60.9
Seville;-28.8,19.2,74.6
Shanghai;-36.3,16.7,66.6
Singapore;-18.1,27.0,78.7
Skopje;-40.8,12.4,61.8
Sochi;-32.9,14.2,64.4
Sofia;-39.6,10.6,61.0
Sokoto;-21.3,28.0,77.8
Split;-32.6,16.1,64.1
St. John's;-43.9,5.0,56.0
St. Louis;-36.8,13.9,64.5
Stockholm;-42.2,6.6,53.4
Surabaya;-21.7,27.1,78.1
Suva;-24.0,25.6,73.9
Suwałki;-41.0,7.2,53.4
Sydney;-33.2,17.7,70.0
Ségou;-20.2,28.0,76.2
Tabora;-27.6,23.0,75.1
Tabriz;-39.4,12.6,67.7
Taipei;-30.8,23.0,72.6
Tallinn;-41.8,6.4,58.5
Tamale;-22.8,27.9,77.8
Tamanrasset;-24.3,21.7,73.9
Tampa;-24.9,22.9,73.4
Tashkent;-34.5,14.8,65.8
Tauranga;-33.3,14.8,68.2
Tbilisi;-35.9,12.9,61.2
Tegucigalpa;-26.1,21.7,72.1
Tehran;-30.5,17.0,67.2
Tel Aviv;-32.0,20.0,70.8
Thessaloniki;-34.7,16.0,65.0
Thiès;-25.3,24.0,74.4
Tijuana;-35.8,17.8,71.0
Timbuktu;-19.5,28.0,75.7
Tirana;-36.7,15.2,66.2
Toamasina;-26.0,23.4,70.0
Tokyo;-35.5,15.4,63.1
Toliara;-24.7,24.1,75.7
Toluca;-40.3,12.4,62.6
Toronto;-41.7,9.4,59.0
Tripoli;-29.1,20.0,70.5
Tromsø;-44.6,2.9,56.6
Tucson;-30.6,20.9,72.9
Tunis;-31.9,18.4,69.9
Ulaanbaatar;-48.8,-0.4,59.1
Upington;-26.5,20.4,67.8
Vaduz;-39.1,10.1,60.1
Valencia;-31.3,18.3,65.4
Valletta;-28.2,18.8,68.7
Vancouver;-38.8,10.4,58.9
Veracruz;-25.3,25.4,75.9
Vienna;-47.0,10.4,60.2
Vientiane;-24.2,25.9,74.5
Villahermosa;-21.9,27.1,77.2
Vilnius;-41.9,6.0,57.6
Virginia Beach;-33.5,15.8,64.9
Vladivostok;-44.7,4.9,56.9
Warsaw;-47.9,8.5,57.3
Washington, D.C.;-36.6,14.6,72.7
Wau;-33.7,27.8,87.8
Wellington;-35.9,12.9,65.9
Whitehorse;-48.3,-0.1,49.3
Wichita;-35.5,13.9,68.0
Willemstad;-24.9,28.0,78.6
Winnipeg;-50.3,3.0,54.1
Wrocław;-43.9,9.6,60.9
Xi'an;-35.6,14.1,64.1
Yakutsk;-58.9,-8.8,39.9
Yangon;-30.2,27.5,75.5
Yaoundé;-25.0,23.8,72.8
Yellowknife;-55.9,-4.3,45.1
Yerevan;-39.2,12.4,65.7
Yinchuan;-44.5,9.0,56.5
Zagreb;-36.7,10.7,62.0
Zanzibar City;-26.1,26.0,77.0
Zürich;-38.2,9.3,57.5
Ürümqi;-43.5,7.4,55.7
İzmir;-36.6,17.9,64.3
target/release/one_billion_row_challenge_polars  104.88s user 116.69s system 68% cpu 5:23.15 total
```