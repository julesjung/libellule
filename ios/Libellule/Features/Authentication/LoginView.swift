//
//  LoginView.swift
//  Libellule
//
//  Created by Jules on 05/08/2026.
//

import SwiftUI
import LibelluleKit

struct LoginView: View {
    @Environment(\.syncService) private var syncService
    
    @State private var url: String = ""
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var error: AppError?
    @State private var loading = false
    
    @Binding var appParameters: AppParameters?
    
    @FocusState private var focusedField: Field?

    private enum Field {
        case url
        case username
        case password
    }

    var body: some View {
        NavigationStack {
            VStack {
                HStack {
                    Image(systemName: "link")
                    TextField("URL de l'instance PRONOTE", text: $url)
                        .focused($focusedField, equals: .url)
                        .textContentType(.URL)
                        .submitLabel(.continue)
                        .onSubmit {
                            focusedField = .username
                        }
                }
                .padding()
                .textFieldStyle(.plain)
                .glassEffect()
                
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
                                await login()
                            }
                        }
                }
                .padding()
                .glassEffect()
                
                Button {
                    Task {
                        await login()
                    }
                } label: {
                    HStack {
                        if loading {
                            ProgressView()
                        }
                        Text("Connexion")
                    }
                }
                .disabled(loading || url.isEmpty || username.isEmpty || password.isEmpty)
                .buttonStyle(.glassProminent)
                .controlSize(.large)
                .buttonSizing(.flexible)
            }
            .autocorrectionDisabled(true)
            .textInputAutocapitalization(.never)
            .textFieldStyle(.plain)
            .padding()
            .alert(error: $error) {
                Button(role: .confirm) {
                    error = nil
                }
            }
            .navigationTitle("Connexion")
        }
    }
    
    func login() async {
        do {
            loading = true
            appParameters = try await syncService!.login(url: url, username: username, password: password)
            if let data = try? JSONEncoder().encode(appParameters) {
                UserDefaults.standard.set(data, forKey: "app")
            }
            try? KeychainService.shared.save(value: username, forKey: "username")
            try? KeychainService.shared.save(value: password, forKey: "password")
        } catch {
            self.error = error
        }
    }
}
