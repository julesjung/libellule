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
    @State private var user: User?
    
    var body: some View {
        VStack {
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
            } else {
                switch client!.status {
                case .disconnected, .connecting:
                    ProgressView("Connexion à l'instance PRONOTE")
                case .connected:
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
                case .authenticating:
                    ProgressView("Authentification")
                case .authenticated, .requesting:
                    if user == nil {
                        ProgressView("Chargement des données")
                        .task {
                            user = try! await client!.userInformation()
                        }
                    } else {
                        UserView(user: user!)
                    }
                }
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
