//
//  ContentView.swift
//  Pronote
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import PronoteKit

struct ContentView: View {
    @State private var client: ObservableClient?
    @State private var url: String = ""
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var gradesData: GradesData?
    
    var body: some View {
        NavigationStack {
            if client == nil {
                VStack {
                    HStack {
                        Image(systemName: "link")
                        TextField("URL de l'instance PRONOTE", text: $url)
                            .autocorrectionDisabled(true)
                            .textInputAutocapitalization(.never)
                            .keyboardType(.URL)
                            .textFieldStyle(.plain)
                    }
                    .padding()
                    .glassEffect()
                    Button {
                        Task {
                            client = try! await ObservableClient(instanceUrl: url)
                            
                            try! await client!.connect()
                        }
                    } label: {
                        Text("Suivant")
                    }
                    .disabled(url.isEmpty)
                    .buttonStyle(.glassProminent)
                }
                .padding()
            } else {
                switch client!.status {
                case .disconnected:
                    ProgressView("Connexion à l'instance PRONOTE")
                case .connected:
                    VStack {
                        Spacer()
                        HStack {
                            Image(systemName: "person")
                            TextField("Nom d'utilisateur", text: $username)
                                .autocorrectionDisabled(true)
                                .textInputAutocapitalization(.never)
                                .textFieldStyle(.plain)
                        }
                        .padding()
                        .glassEffect()
                        HStack {
                            Image(systemName: "lock")
                            SecureField("Mot de passe", text: $password)
                                .autocorrectionDisabled(true)
                                .textInputAutocapitalization(.never)
                                .textFieldStyle(.plain)
                        }
                        .padding()
                        .glassEffect()
                        
                        Button {
                            Task {
                                try! await client?.authenticate(username: username, password: password)
                            }
                        } label: {
                            Text("Connexion")
                        }
                        .disabled(username.isEmpty || password.isEmpty)
                        .buttonStyle(.glassProminent)
                        Spacer()
                    }
                    .padding()
                case .authenticated:
                    ProgressView("Chargement de l'utilsateur")
                        .task {
                            try! await client!.loadUser()
                        }
                case .ready:
                    if gradesData == nil {
                        ProgressView("Chargement des notes")
                            .task {
                                gradesData = try! await client!.getGrades()
                            }
                    } else {
                        NavigationLink("Voir les notes") {
                            GradesView(gradesData: gradesData!)
                        }
                        .buttonStyle(.glass)
                    }
                }
            }
        }
    }
}

#Preview {
    ContentView()
}
