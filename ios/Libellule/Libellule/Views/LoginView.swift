//
//  LoginView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct LoginView: View {
    var instance: Instance
    @Binding var state: AppState
    @State private var username: String = ""
    @State private var password: String = ""

    var body: some View {
        VStack {
            Text(instance.label())
                .font(.headline)
            HStack {
                Image(systemName: "person.crop.circle")
                TextField("Nom d'utilisateur", text: $username)
            }
            .padding()
            .glassEffect()

            HStack {
                Image(systemName: "key")
                SecureField("Mot de passe", text: $password)
            }
            .padding()
            .glassEffect()

            Button {
                state = .authenticating(instance: instance, username: username, password: password)
            } label: {
                Text("Connexion")
            }
            .disabled(username.isEmpty || password.isEmpty)
            .buttonStyle(.glassProminent)
            .controlSize(.large)
            .buttonSizing(.flexible)
        }
        .autocorrectionDisabled(true)
        .textInputAutocapitalization(.never)
        .textFieldStyle(.plain)
        .padding()
    }
}
