//
//  ContentView.swift
//  Pronote
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import PronoteKit

struct ContentView: View {
    @State private var client: Client?
    @State private var parameters: ParametersRecord?
    @State private var username: String = ""
    @State private var password: String = ""
    @State private var fullname: String?
    
    var body: some View {
        VStack {
            if parameters == nil {
                ProgressView("Connecting to PRONOTE instance")
            } else if fullname == nil {
                Spacer()
                TextField("Username", text: $username)
                    .autocorrectionDisabled(true)
                    .textInputAutocapitalization(.never)
                    .textFieldStyle(.roundedBorder)
                SecureField("Password", text: $password)
                    .textFieldStyle(.roundedBorder)
                Button {
                    Task {
                        if client != nil {
                            fullname = try! await client?.authenticate(username: username, password: password)
                        }
                    }
                } label: {
                    Text("Log In")
                }
                .disabled(username.isEmpty || password.isEmpty)
                .buttonSizing(.flexible)
                .buttonStyle(.glassProminent)
                Spacer()
            } else {
                Text("Hello \(fullname ?? "")")
            }
        }
        .padding()
        .task {
            client = try! await Client(instanceUrl: "https://demo.index-education.net/pronote/")
            
            parameters = try! await client?.connect()
        }
    }
}

#Preview {
    ContentView()
}
