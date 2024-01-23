# 1BRC in rust

[The One Billion Row Challenge](https://github.com/gunnarmorling/1brc).

At this point in time this is a rather naive implementation of a solution to the 1BRCthe in rust with rayon for parallelism.
On my MacBook Air (M2, 2023) it takes ~142s to complete. Memory usage peaks at ~26GB, which means swapping is taking place.

## Usage
```bash
cargo build --release && time cat measurements-1000000000.txt | pv --rate | target/release/one_billion_row_challenge  
    Finished release [optimized] target(s) in 0.02s
[1.85GiB/s]
Ottawa;6.6,-42.3,60.2
Beijing;12.9,-35.7,62.8
Kinshasa;25.3,-28.6,76.6
Niigata;13.9,-38,60.7
Dampier;26.4,-22.6,77.5
Ashgabat;17.1,-31,67.6
Berlin;10.3,-40.4,60.1
Honolulu;25.4,-22.9,74.2
Napoli;15.9,-32.2,62.5
Las Palmas de Gran Canaria;21.2,-30.1,70.2
Dikson;-11.1,-63.9,37.9
Hargeisa;21.7,-30.8,67.5
Fairbanks;-2.3,-50.9,50.3
Taipei;23,-30.8,72.6
Edmonton;4.2,-46.1,59.8
Tel Aviv;20,-32,70.8
Durban;20.6,-29.6,71.5
Vaduz;10.1,-39.1,60.1
Benghazi;19.9,-28.4,69.1
Thiès;24,-25.3,74.4
Astana;3.5,-51.5,54
Baku;15.1,-35.9,65.6
Livingstone;21.8,-27.9,76.4
Cracow;9.3,-37.4,63
Marrakesh;19.6,-30.3,68.3
Hat Yai;27,-22.4,76.8
Juba;27.8,-22.8,80.2
Murmansk;0.6,-51,54.4
Entebbe;21,-27.7,72.3
Honiara;26.5,-23.1,74.9
Bulawayo;18.9,-33.5,69.5
Ghanzi;21.4,-28.4,69.8
Tallinn;6.4,-41.8,58.5
Wellington;12.9,-35.9,65.9
Jos;22.8,-26.4,72.2
Luanda;25.8,-27.4,76.5
Tbilisi;12.9,-35.9,61.2
Lomé;26.9,-23.2,73.3
Mango;28.1,-19.9,75.6
Seoul;12.5,-37.4,60.9
Oranjestad;28.1,-22.9,76.4
Gjoa Haven;-14.4,-68.7,34.2
Oklahoma City;15.9,-35,68.5
Saint Petersburg;5.8,-44.8,56.7
Belgrade;12.5,-39.4,59.8
Lisbon;17.5,-33.1,67.1
Fukuoka;17,-32,66
Bloemfontein;15.6,-37.4,64.4
Pointe-Noire;26.1,-24.6,74.7
Virginia Beach;15.8,-33.5,64.9
Oslo;5.7,-46.5,55.4
Skopje;12.4,-40.8,61.8
Changsha;17.4,-31.1,70.5
Chittagong;25.9,-23.3,75.9
Timbuktu;28,-19.5,75.7
San Jose;16.4,-39,68.2
Guadalajara;20.9,-31.6,70.5
Garissa;29.3,-23.7,82.4
Tripoli;20,-29.1,70.5
Tabora;23,-27.6,75.1
Alexandria;20,-31.1,75.7
Split;16.1,-32.6,64.1
El Paso;18.1,-31,68.6
Valletta;18.8,-28.2,68.7
Lhasa;7.6,-40.6,57.1
Mandalay;28,-18.6,78.6
Erbil;19.5,-30.3,78.4
Anchorage;2.8,-46.9,53.9
Montreal;6.8,-44.7,59.8
Malabo;26.3,-26.5,78.3
Odesa;10.7,-40.7,57.2
Brussels;10.5,-41,61.8
Hanga Roa;20.5,-27.6,67.9
Baltimore;13.1,-38.7,64.7
Nicosia;19.7,-29.6,71
Minneapolis;7.8,-40.8,57.1
Dakar;24,-26.2,73.2
Havana;25.2,-25.9,72.8
Kuala Lumpur;27.3,-22.9,81.2
Atlanta;17,-31.5,67.3
Seville;19.2,-28.8,74.6
Veracruz;25.4,-25.3,75.9
Dublin;9.8,-45.5,56.9
Willemstad;28,-24.9,78.6
Vladivostok;4.9,-44.7,56.9
Marseille;15.8,-35.3,72.7
Melbourne;15.1,-30.9,63.8
Dolisie;24,-25,77.6
San Salvador;23.1,-24.9,70.1
Praia;24.4,-26.6,77.3
Edinburgh;9.3,-41.8,58.8
Kampala;20,-27.5,69.7
Tamale;27.9,-22.8,77.8
Libreville;25.9,-27.2,78.6
Makassar;26.7,-25.9,74.7
Heraklion;18.9,-29.9,68.2
Baguio;19.5,-28.2,69.8
Jacksonville;20.3,-26.5,71.9
Zanzibar City;26,-26.1,77
Riga;6.2,-44.5,52.6
Tucson;20.9,-30.6,72.9
Copenhagen;9.1,-39.9,56.2
Yakutsk;-8.8,-58.9,39.9
Memphis;17.2,-31.1,69.2
Erzurum;5.1,-43.4,58.9
Rostov-on-Don;9.9,-39.3,58.4
Indianapolis;11.8,-35.5,61.4
Dili;26.6,-26.2,76.1
Yellowknife;-4.3,-55.9,45.1
Lake Tekapo;8.7,-40.3,55.7
Yerevan;12.4,-39.2,65.7
Medan;26.5,-25.9,79.1
Upington;20.4,-26.5,67.8
Valencia;18.3,-31.3,65.4
Belize City;26.7,-27,75.3
Milan;13,-35.9,61.6
Saskatoon;3.3,-46.8,54
Djibouti;29.9,-19.2,79
Makurdi;26,-24.8,77.6
Pretoria;18.2,-35.2,68.2
Sofia;10.6,-39.6,61
Anadyr;-6.9,-56.5,41
Stockholm;6.6,-42.2,53.4
Las Vegas;20.3,-28.6,70.1
Mombasa;26.3,-25.7,78.2
Ndola;20.3,-30.7,68.8
Accra;26.4,-21.4,76
Maun;22.4,-28.4,71.6
Gagnoa;26,-22.4,74.3
Helsinki;5.9,-45.9,52.1
Wau;27.8,-33.7,87.8
Bata;25.1,-23.9,77.1
Nairobi;17.8,-28.1,69.4
Nassau;24.6,-25,73.4
Boise;11.4,-35.9,59.9
Cairns;25,-30.4,79.9
Seattle;11.3,-40.5,60.9
Sapporo;8.9,-41.2,56.8
Douala;26.7,-26.1,74
Toluca;12.4,-40.3,62.6
Ngaoundéré;22,-25.2,71.2
Milwaukee;8.9,-41.3,61
Managua;27.3,-21.7,79.8
La Ceiba;26.2,-20.6,74.8
Nuuk;-1.4,-52.9,51.9
Athens;19.2,-30,69.2
Algiers;18.2,-31,66.2
Kathmandu;18.3,-30.3,65.1
Johannesburg;15.5,-35.1,63.8
Cairo;21.4,-27.1,70.8
Birao;26.5,-28,76.1
Tehran;17,-30.5,67.2
Los Angeles;18.6,-34.4,65.4
Suva;25.6,-24,73.9
Baghdad;22.8,-28.2,73.1
Conakry;26.4,-22.2,82.8
Bridgetown;27,-27.4,77.5
Manila;28.4,-20.1,77.4
Jerusalem;18.3,-32.7,70.3
Portland (OR);12.4,-36.7,61.9
Dhaka;25.9,-26.9,78.5
Mzuzu;17.7,-32.7,70.4
Ouarzazate;18.9,-31.5,69.4
St. Louis;13.9,-36.8,64.5
Cape Town;16.2,-32.4,67.3
Riyadh;26,-24.6,79.2
New Orleans;20.7,-32.6,68.5
Kandi;27.7,-22.7,80.3
Dallas;19,-30.1,70.1
Whitehorse;-0.1,-48.3,49.3
Ifrane;11.4,-42.2,61.2
Lodwar;29.3,-23.2,79.3
Bangkok;28.6,-20.6,79.7
Budapest;11.3,-36.8,60.7
Darwin;27.6,-22.6,77.7
Istanbul;13.9,-33.1,63.7
Monterrey;22.3,-27.4,71.8
Gaborone;21,-28.7,72.6
Blantyre;22.2,-25.1,72.1
Lake Havasu City;23.7,-25.1,71.7
Warsaw;8.5,-47.9,57.3
Louisville;13.9,-35.9,61.8
Bamako;27.8,-21.3,81.7
Phnom Penh;28.3,-23.2,83.2
Tromsø;2.9,-44.6,56.6
San Juan;27.2,-23.6,82
Odienné;26,-23.6,77.1
Canberra;13.1,-38.6,63.1
Xi'an;14.1,-35.6,64.1
Tunis;18.4,-31.9,69.9
Abéché;29.4,-18.9,78.1
Da Lat;17.9,-32.4,66
Abidjan;26,-24.1,74.6
Cotonou;27.2,-21,81.2
Pontianak;27.7,-20.5,76.2
Lahore;24.3,-23.5,75.5
Saint-Pierre;5.7,-45.9,62
Barcelona;18.2,-28.7,72
Manama;26.5,-26.5,74.7
Suwałki;7.2,-41,53.4
Tauranga;14.8,-33.3,68.2
Bilbao;14.7,-36.2,66.5
Charlotte;16.1,-34.4,70.1
Phoenix;23.9,-24,75.8
Naha;23.1,-25.2,72.7
Palmerston North;13.2,-35.6,62
Port Moresby;26.9,-20.5,76.4
Halifax;7.5,-41,58.5
Port Vila;24.3,-27,77.4
Hanoi;23.6,-28.1,76.3
Calgary;4.4,-47.6,53.6
Wrocław;9.6,-43.9,60.9
Aden;29.1,-19.4,80.8
Lyon;12.5,-40.3,62.9
Vientiane;25.9,-24.2,74.5
Detroit;10,-37.9,58.8
Palembang;27.3,-24.9,81.6
Hiroshima;16.3,-30.8,64.9
Austin;20.7,-29.9,73
Tijuana;17.8,-35.8,71
Panama City;28,-22.4,78.7
Bujumbura;23.8,-26,73.3
Ho Chi Minh City;27.4,-20.8,75.9
Port Sudan;28.4,-21.1,78.5
Washington, D.C.;14.6,-36.6,72.7
Lagos;26.8,-24,75.9
Bratislava;10.5,-45.4,62.3
Chongqing;18.6,-30.4,69
New York City;12.9,-35.2,64.9
Singapore;27,-18.1,78.7
Colombo;27.4,-20.3,78
Busan;15,-37.1,64.4
Sana'a;20,-30.6,70.9
Amsterdam;10.2,-39.3,62.4
Kano;26.4,-23.2,77.3
Chișinău;10.2,-44.9,59.1
Alice Springs;21,-31.2,71.1
Winnipeg;3,-50.3,54.1
Brisbane;21.4,-27.7,70.4
Iqaluit;-9.3,-59.9,39.2
Da Nang;25.8,-23.9,74.6
Salt Lake City;11.6,-40.9,57.7
Harbin;5,-46.4,57.5
Rangpur;24.4,-23.4,74.2
Mumbai;27.1,-25.2,72.6
Vancouver;10.4,-38.8,58.9
Nouadhibou;21.3,-28.8,73.2
Yinchuan;9,-44.5,56.5
Kingston;27.4,-23.7,78.7
Napier;14.6,-35.3,66.7
Miami;24.9,-24.3,72.3
New Delhi;25,-24,72.3
Arkhangelsk;1.3,-48.3,49.5
Cabo San Lucas;23.9,-26.3,73.9
Sarajevo;10.1,-37.3,59.4
Kuwait City;25.7,-24.2,73.9
Shanghai;16.7,-36.3,66.6
Toronto;9.4,-41.7,59
Porto;15.7,-32.7,65.7
Malé;28,-21.2,76.1
Tabriz;12.6,-39.4,67.7
Tokyo;15.4,-35.5,63.1
Vienna;10.4,-47,60.2
Kunming;15.7,-33.8,67.5
Vilnius;6,-41.9,57.6
Bergen;7.7,-42.3,57.5
Launceston;13.1,-36.1,66.8
Kuopio;3.4,-50.1,50.9
Fresno;17.9,-30.8,65.9
Batumi;14,-38.9,67.2
Moscow;5.8,-44.3,58.8
Oulu;2.7,-48,50.9
Kankan;26.5,-22,76
Dushanbe;14.7,-36,66.4
Sochi;14.2,-32.9,64.4
Podgorica;15.3,-37.5,64.2
Parakou;26.8,-25.4,78.4
San José;22.6,-27,74.6
Burnie;13.1,-36.8,63.9
Guangzhou;22.4,-31.4,71.1
Reykjavík;4.3,-50.2,57.2
Addis Ababa;16,-32.8,66.4
Kolkata;26.7,-26.5,78.8
Dunedin;11.1,-39.8,59.5
Mexico City;17.5,-30.9,69.5
Zürich;9.3,-38.2,57.5
Ulaanbaatar;-0.4,-48.8,59.1
San Francisco;14.6,-39.4,65.7
Reggane;28.3,-19.8,77.3
Denpasar;23.7,-28,73.4
Flores,  Petén;26.4,-21.9,77.1
La Paz;23.7,-24,73.2
Guatemala City;20.4,-30.8,73
Brazzaville;25,-28.2,75.1
Tashkent;14.8,-34.5,65.8
Dar es Salaam;25.8,-24.4,74.7
Sydney;17.7,-33.2,70
N'Djamena;28.3,-23.8,77.7
Tirana;15.2,-36.7,66.2
Santo Domingo;25.9,-23.2,73
Kyiv;8.4,-43.2,59.4
George Town;27.9,-21.4,80
Khartoum;29.9,-20.2,78.3
Minsk;6.7,-41.5,54.1
Philadelphia;13.2,-36.5,65.8
Sokoto;28,-21.3,77.8
Ljubljana;10.9,-38.6,60.8
Chiang Mai;25.8,-28.1,74.5
Almaty;10,-39,61.9
Petropavlovsk-Kamchatsky;1.9,-47.6,53
Albuquerque;14,-36.7,66.9
Hobart;12.7,-33.4,62.6
İzmir;17.9,-36.6,64.3
Tampa;22.9,-24.9,73.4
Irkutsk;1,-46.7,52.7
Ankara;12,-35.6,59.1
Bordeaux;14.2,-38.5,63.2
Adelaide;17.3,-32.8,70
Rome;15.2,-31.8,71.6
Antsiranana;25.2,-23.3,77.2
Novosibirsk;1.7,-46.9,51
Palm Springs;24.5,-26.8,78.7
Luxembourg City;9.3,-38.7,63.6
Auckland;15.2,-33.2,63.9
Kansas City;12.5,-36.6,61.5
Toamasina;23.4,-26,70
Dubai;26.9,-24,76.7
Bucharest;10.8,-36.9,59.3
Dodoma;22.7,-25.1,80.2
Tegucigalpa;21.7,-26.1,72.1
Lubumbashi;20.8,-29.1,71.9
Hamburg;9.7,-45.7,60.1
Ouahigouya;28.6,-20.4,76.1
Zagreb;10.7,-36.7,62
Prague;8.4,-41.6,62.7
Yaoundé;23.8,-25,72.8
Villahermosa;27.1,-21.9,77.2
Lviv;7.8,-41.3,61.7
Surabaya;27.1,-21.7,78.1
Mogadishu;27.1,-22.4,75.9
Chicago;9.8,-38.8,60.8
Omaha;10.6,-40.8,59.3
Pittsburgh;10.8,-40.6,61.3
Bangui;26,-26.1,74.2
Mek'ele;22.7,-28.7,70.1
San Antonio;20.8,-28.9,70.5
Ahvaz;25.4,-23.9,73.2
Mexicali;23.1,-23.5,73.5
Nouakchott;25.7,-24.4,76.3
Sacramento;16.3,-32.5,66
Bissau;27,-27,74.2
Nashville;15.4,-33.4,68.3
Antananarivo;17.9,-34.2,70
Banjul;26,-23.9,79.4
San Diego;17.8,-31,66.7
Columbus;11.7,-44.2,59.4
Mahajanga;26.3,-23.3,78.6
Kyoto;15.8,-34.2,70.1
Madrid;15,-34.6,65.4
Bishkek;11.3,-37,61.7
Asmara;15.6,-33.7,63.3
Kabul;12.1,-44.5,61.9
Christchurch;12.2,-36,63.2
Fianarantsoa;17.9,-35.3,69.1
Maputo;22.8,-27.4,71.6
Bouaké;26,-23,75.3
Andorra la Vella;9.8,-44.1,56.9
Rabat;17.2,-38.5,69.8
Assab;30.5,-21.1,80.7
Niamey;29.3,-24.8,77.5
London;11.3,-41.2,62
Abha;18,-33.9,67.5
Tamanrasset;21.7,-24.3,73.9
Denver;10.4,-40.4,59
Houston;20.8,-26.9,74.2
Muscat;28,-23.5,76
Toliara;24.1,-24.7,75.7
Paris;12.3,-37.3,63.1
Perth;18.7,-29.2,66.4
City of San Marino;11.8,-35.7,65.2
Nakhon Ratchasima;27.3,-22.2,86
Beirut;20.9,-28.4,69.6
Ürümqi;7.4,-43.5,55.7
Garoua;28.3,-20.5,79.3
Lusaka;19.9,-28.2,77.8
Karachi;26,-20.8,74.1
Yangon;27.5,-30.2,75.5
Moncton;6.1,-49.7,55.1
Bosaso;30,-20.4,79.5
Jakarta;26.7,-22.6,74.5
Port-Gentil;26,-29.9,73.6
Gangtok;15.2,-35.2,68.3
Harare;18.4,-38.9,69.3
Damascus;17,-31.5,65.3
Pyongyang;10.8,-44.9,65.6
Alexandra;11,-36,61.2
Gabès;19.5,-32.1,70.1
Thessaloniki;16,-34.7,65
Wichita;13.9,-35.5,68
Palermo;18.5,-30.9,68.8
Kumasi;26,-24.7,73.5
St. John's;5,-43.9,56
Karonga;24.4,-23.9,71.6
Ségou;28,-20.2,76.2
Frankfurt;10.6,-44.3,68.4
Hong Kong;23.3,-25.3,74.4
Hamilton;13.8,-35.4,61.8
Ouagadougou;28.3,-27.3,80.9
Boston;10.9,-37.4,59.4
Roseau;26.2,-22.9,74.8
Monaco;16.4,-33.2,64.4
Jayapura;27,-19,75.6
Chihuahua;18.6,-32.3,65.6
Number of stations: 413
cat measurements-1000000000.txt  0.23s user 3.45s system 52% cpu 6.930 total
pv --rate  0.57s user 2.78s system 48% cpu 6.930 total
target/release/one_billion_row_challenge  141.10s user 27.03s system 350% cpu 48.031 total
```