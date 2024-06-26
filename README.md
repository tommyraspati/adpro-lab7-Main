# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [v] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [v] Commit: `Create Subscriber model struct.`
    -   [v] Commit: `Create Notification model struct.`
    -   [v] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [v] Commit: `Implement add function in Subscriber repository.`
    -   [v] Commit: `Implement list_all function in Subscriber repository.`
    -   [v] Commit: `Implement delete function in Subscriber repository.`
    -   [v] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

- Dalam diagram Observer pattern yang dijelaskan oleh buku Head First Design Patterns, Subscriber didefinisikan sebagai interface. Penggunaan interface atau trait dalam Rust sangat berguna jika ada berbagai jenis Subscriber, yang memungkinkan kita untuk menambahkan jenis-jenis Subscriber baru dengan mudah. Hal ini sesuai dengan Open-Closed Principle, yang menyarankan bahwa perangkat lunak harus terbuka untuk ekstensi tapi tertutup untuk modifikasi. Namun, untuk kasus BambangShop saat ini, jika semua Subscriber bersifat seragam dan tidak memerlukan variasi dalam implementasi, penggunaan satu Model struct mungkin sudah cukup.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?
- Menggunakan DashMap lebih disarankan daripada Vec dalam kasus ini, terutama karena id di Program dan url di Subscriber harus unik. DashMap memungkinkan penyimpanan dan akses data secara efisien melalui key-value pair, yang memudahkan dalam pencarian, penambahan, dan penghapusan data dengan cepat. Dibandingkan dengan Vec, di mana kita perlu melakukan iterasi untuk mencari atau menghapus item, DashMap lebih efektif dan efisien untuk menjaga uniknya setiap elemen.
3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?
- Dalam konteks Rust yang memiliki ketatnya keamanan pada konkurensi, penggunaan DashMap adalah pilihan yang tepat karena DashMap sudah dirancang untuk penggunaan konkuren yang aman, menghindari masalah race condition dan deadlock. Ini lebih tepat daripada menggunakan HashMap standar yang tidak aman untuk akses konkuren tanpa mekanisme sinkronisasi yang tepat. Meskipun demikian, penggunaan pattern Singleton masih relevan untuk memastikan bahwa hanya ada satu instance dari DashMap yang digunakan (dalam hal ini adalah SUBSCRIBERS) di seluruh program, yang menjamin konsistensi dan keamanan akses ke data bersama.

#### Reflection Publisher-2
1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?
- Pemisahan 'Service' dan 'Repository' dari Model sangat penting untuk mematuhi Single Responsibility Principle (SRP) dalam prinsip desain perangkat lunak. Dengan memisahkan 'Service' dan 'Repository', kita dapat memastikan bahwa setiap komponen memiliki tanggung jawab yang jelas dan terbatas. 'Service' bertanggung jawab untuk mengelola logika bisnis dan mengoperasikan data yang diperoleh dari 'Repository', sementara 'Repository' bertugas mengelola akses dan operasi data pada database. Pendekatan ini membantu dalam memelihara modularitas dan kejelasan dalam struktur kode.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?
- Jika kita hanya menggunakan Model untuk menangani logika bisnis dan akses data, ini dapat mengakibatkan tingkat ketergantungan (coupling) yang tinggi antar komponen dalam aplikasi. Setiap Model mungkin menjadi sangat kompleks karena menangani lebih dari satu tanggung jawab, membuatnya sulit untuk dipelihara dan diadaptasi dengan perubahan. Interaksi antar Model seperti Program, Subscriber, dan Notification bisa menjadi rumit dan susah untuk dikelola, karena setiap Model akan cenderung menjadi terlalu besar dan melakukan terlalu banyak tugas.
3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.
- Postman adalah alat yang sangat berguna untuk menguji API. Dengan menggunakan Postman, saya dapat dengan mudah menguji endpoint API untuk memastikan mereka mengembalikan respons yang diharapkan. Fitur 'Collections' di Postman sangat membantu dalam mengorganisir berbagai request API yang saya uji, memungkinkan saya untuk mengelola dan menjalankan serangkaian tes secara sistematis. Alat ini akan sangat berguna tidak hanya untuk proyek kelompok saya saat ini tetapi juga untuk proyek rekayasa perangkat lunak di masa depan, karena memudahkan proses debugging dan validasi fungsionalitas API.

#### Reflection Publisher-3
1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?
- Dalam tutorial ini, kita menggunakan model push dari Observer Pattern. Dalam model ini, setiap kali terjadi perubahan pada produk (seperti pembuatan, penghapusan, atau pembaruan), penerbit (publisher) akan secara aktif mengirimkan data terkait perubahan tersebut kepada semua pelanggan (subscribers) yang berlangganan tipe produk tersebut.
2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)
- Keuntungan menggunakan model pull: Salah satu keuntungan utama dari model pull adalah bahwa informasi tentang pelanggan tidak terpapar kepada penerbit. Ini menambah tingkat privasi dan keamanan karena pelanggan yang mengontrol kapan mereka mengambil data, sehingga mengurangi risiko kebocoran data.
- Kerugian menggunakan model pull: Kerugiannya adalah dapat terjadi penundaan dalam menerima pembaruan karena pelanggan harus secara aktif meminta informasi dari penerbit. Ini bisa mengakibatkan pelanggan tidak segera mendapatkan data terbaru, terutama dalam situasi di mana informasi harus cepat dikomunikasikan.
3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.
- Jika kita memutuskan untuk tidak menggunakan multi-threading dalam proses pengiriman notifikasi, proses tersebut akan berjalan secara sinkronus dan bisa menjadi sangat lambat. Ini karena sistem akan menunggu selesainya pengiriman notifikasi kepada satu pelanggan sebelum beralih ke pelanggan berikutnya. Dalam kasus di mana ada banyak pelanggan, ini bisa menyebabkan penundaan yang signifikan dalam pengiriman notifikasi, berpotensi mengurangi efektivitas dan responsivitas dari sistem notifikasi itu sendiri.

