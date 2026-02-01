/*
 * Copyright 2026 seasnail1
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import './styles/style.scss';

import CredentialLoginPage from './pages/login'
import HomePage from './pages/home.tsx'

import { Routes, Route } from 'react-router-dom';

function App() {
    return (
        <Routes>
            <Route path="/home" element={<HomePage/>} />
            <Route path="/login" element={<CredentialLoginPage/>} />
        </Routes>
    );
}

export default App;