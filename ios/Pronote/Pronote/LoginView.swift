//
//  LoginView.swift
//  Pronote
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import PronoteKit

struct LoginView: View {
    var instance: Instance
    @Binding var client: Client?
    @Binding var state: AppState
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var authenticating: Bool = false

    var body: some View {
        if authenticating == false {
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
                        authenticating = true
                        client = try! await Client(instance: instance, password: username, username: password)
                        state = .home
                    }
                } label: {
                    Text("Connexion")
                }
                .disabled(username.isEmpty || password.isEmpty)
                .buttonStyle(.glassProminent)
                Spacer()
            }
            .padding()
        } else {
            ProgressView("Authentification")
        }
    }
}
