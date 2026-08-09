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
    
    @FocusState private var focusedField: Field?

    private enum Field {
        case username
        case password
    }

    var body: some View {
        VStack {
            Text(instance.label())
                .font(.headline)
            HStack {
                Image(systemName: "person.crop.circle")
                TextField("Nom d'utilisateur", text: $username)
                    .focused($focusedField, equals: .username)
                    .textContentType(.username)
                    .submitLabel(.continue)
                    .onSubmit {
                        focusedField = .password
                    }
            }
            .padding()
            .glassEffect()

            HStack {
                Image(systemName: "key")
                SecureField("Mot de passe", text: $password)
                    .focused($focusedField, equals: .password)
                    .textContentType(.password)
                    .submitLabel(.done)
                    .onSubmit {
                        login()
                    }
            }
            .padding()
            .glassEffect()

            Button("Connexion") {
                login()
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
    
    private func login() {
        focusedField = nil

        state = .authenticating(
            instance: instance,
            username: username,
            password: password
        )
    }
}
