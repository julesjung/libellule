//
//  LoginView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct LoginView: View {
    @Environment(SessionStore.self) private var session
    let instance: Instance
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
                        Task {
                            await session.login(instance: instance, username: username, password: password)
                        }
                    }
            }
            .padding()
            .glassEffect()
            
            Button("Connexion") {
                Task {
                    await session.login(instance: instance, username: username, password: password)
                }
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
