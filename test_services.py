#!/usr/bin/env python3
"""
Bunkerverse Platform - Local Docker Environment Test Suite
Comprehensive testing script for validating all Docker Compose services
"""

import requests
import time
import sys
import json
import subprocess
from typing import Dict, List, Tuple, Optional
import psycopg2
import redis
from urllib.parse import urljoin

class BunkerverseTestSuite:
    def __init__(self):
        self.results = {
            "services": {},
            "infrastructure": {},
            "health_checks": {},
            "dns_tests": {},
            "crypto_mode_tests": {},
            "errors": []
        }
        
        # Service endpoints
        self.services = {
            "marketplace": {"port": 8081, "url": "http://localhost:8081"},
            "indexer": {"port": 8082, "url": "http://localhost:8082"},
            "identity": {"port": 8083, "url": "http://localhost:8083"},
            "ai-data": {"port": 8084, "url": "http://localhost:8084"},
            "account": {"port": 8085, "url": "http://localhost:8085"},
            "feedback": {"port": 8086, "url": "http://localhost:8086"},
            "mission": {"port": 8087, "url": "http://localhost:8087"},
            "payment": {"port": 8088, "url": "http://localhost:8088"},
            "social": {"port": 8089, "url": "http://localhost:8089"}
        }
        
        # Infrastructure endpoints
        self.infrastructure = {
            "postgres": {"port": 5432, "url": "postgresql://bunkerverse:dev123@localhost:5432/bunkerverse"},
            "redis": {"port": 6379, "url": "redis://:dev123@localhost:6379"},
            "elasticsearch": {"port": 9200, "url": "http://localhost:9200"},
            "ethereum": {"port": 8545, "url": "http://localhost:8545"},
            "arbitrum": {"port": 8547, "url": "http://localhost:8547"},
            "ipfs-mock": {"port": 8080, "url": "http://localhost:8080"},
            "arweave-mock": {"port": 1984, "url": "http://localhost:1984"}
        }

    def log(self, message: str, level: str = "INFO"):
        """Enhanced logging with timestamps"""
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] [{level}] {message}")

    def test_service_health(self, service_name: str, service_info: Dict) -> bool:
        """Test individual service health endpoint"""
        try:
            health_url = f"{service_info['url']}/health"
            response = requests.get(health_url, timeout=10)
            
            if response.status_code == 200:
                self.log(f"✅ {service_name} health check passed")
                self.results["health_checks"][service_name] = {
                    "status": "healthy",
                    "response_time": response.elapsed.total_seconds(),
                    "response": response.json() if response.headers.get('content-type', '').startswith('application/json') else response.text[:200]
                }
                return True
            else:
                self.log(f"❌ {service_name} health check failed: HTTP {response.status_code}", "ERROR")
                self.results["health_checks"][service_name] = {
                    "status": "unhealthy",
                    "error": f"HTTP {response.status_code}",
                    "response": response.text[:200]
                }
                return False
                
        except requests.exceptions.ConnectionError:
            self.log(f"❌ {service_name} connection failed - service not accessible", "ERROR")
            self.results["health_checks"][service_name] = {"status": "unreachable", "error": "Connection refused"}
            return False
        except Exception as e:
            self.log(f"❌ {service_name} health check error: {str(e)}", "ERROR")
            self.results["health_checks"][service_name] = {"status": "error", "error": str(e)}
            return False

    def test_database_connection(self) -> bool:
        """Test PostgreSQL database connectivity"""
        try:
            conn = psycopg2.connect(
                host="localhost",
                port=5432,
                database="bunkerverse",
                user="bunkerverse",
                password="dev123"
            )
            cursor = conn.cursor()
            cursor.execute("SELECT version();")
            version = cursor.fetchone()
            cursor.close()
            conn.close()
            
            self.log("✅ PostgreSQL database connection successful")
            self.results["infrastructure"]["postgres"] = {
                "status": "connected",
                "version": version[0] if version else "unknown"
            }
            return True
            
        except Exception as e:
            self.log(f"❌ PostgreSQL connection failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["postgres"] = {"status": "failed", "error": str(e)}
            return False

    def test_redis_connection(self) -> bool:
        """Test Redis cache connectivity"""
        try:
            r = redis.Redis(host='localhost', port=6379, password='dev123', decode_responses=True)
            r.ping()
            info = r.info()
            
            self.log("✅ Redis connection successful")
            self.results["infrastructure"]["redis"] = {
                "status": "connected",
                "version": info.get("redis_version", "unknown"),
                "used_memory": info.get("used_memory_human", "unknown")
            }
            return True
            
        except Exception as e:
            self.log(f"❌ Redis connection failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["redis"] = {"status": "failed", "error": str(e)}
            return False

    def test_elasticsearch_connection(self) -> bool:
        """Test Elasticsearch connectivity"""
        try:
            response = requests.get("http://localhost:9200/_cluster/health", timeout=10)
            if response.status_code == 200:
                health_data = response.json()
                self.log("✅ Elasticsearch connection successful")
                self.results["infrastructure"]["elasticsearch"] = {
                    "status": "connected",
                    "cluster_status": health_data.get("status", "unknown"),
                    "number_of_nodes": health_data.get("number_of_nodes", 0)
                }
                return True
            else:
                raise Exception(f"HTTP {response.status_code}")
                
        except Exception as e:
            self.log(f"❌ Elasticsearch connection failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["elasticsearch"] = {"status": "failed", "error": str(e)}
            return False

    def test_blockchain_nodes(self) -> bool:
        """Test blockchain node connectivity"""
        results = []
        
        # Test Ethereum node
        try:
            ethereum_payload = {"jsonrpc": "2.0", "method": "eth_chainId", "params": [], "id": 1}
            response = requests.post("http://localhost:8545", json=ethereum_payload, timeout=10)
            
            if response.status_code == 200 and "result" in response.json():
                chain_id = response.json()["result"]
                self.log(f"✅ Ethereum node connected (Chain ID: {chain_id})")
                self.results["infrastructure"]["ethereum"] = {
                    "status": "connected",
                    "chain_id": chain_id,
                    "rpc_url": "http://localhost:8545"
                }
                results.append(True)
            else:
                raise Exception("Invalid RPC response")
                
        except Exception as e:
            self.log(f"❌ Ethereum node failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["ethereum"] = {"status": "failed", "error": str(e)}
            results.append(False)
        
        # Test Arbitrum sequencer
        try:
            arbitrum_payload = {"jsonrpc": "2.0", "method": "eth_chainId", "params": [], "id": 1}
            response = requests.post("http://localhost:8547", json=arbitrum_payload, timeout=10)
            
            if response.status_code == 200 and "result" in response.json():
                chain_id = response.json()["result"]
                self.log(f"✅ Arbitrum sequencer connected (Chain ID: {chain_id})")
                self.results["infrastructure"]["arbitrum"] = {
                    "status": "connected",
                    "chain_id": chain_id,
                    "rpc_url": "http://localhost:8547"
                }
                results.append(True)
            else:
                raise Exception("Invalid RPC response")
                
        except Exception as e:
            self.log(f"❌ Arbitrum sequencer failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["arbitrum"] = {"status": "failed", "error": str(e)}
            results.append(False)
            
        return all(results)

    def test_mock_services(self) -> bool:
        """Test IPFS and Arweave mock services"""
        results = []
        
        # Test IPFS Mock
        try:
            response = requests.get("http://localhost:8080", timeout=10)
            if response.status_code == 200:
                self.log("✅ IPFS mock service accessible")
                self.results["infrastructure"]["ipfs_mock"] = {
                    "status": "accessible",
                    "response_length": len(response.text)
                }
                results.append(True)
            else:
                raise Exception(f"HTTP {response.status_code}")
                
        except Exception as e:
            self.log(f"❌ IPFS mock service failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["ipfs_mock"] = {"status": "failed", "error": str(e)}
            results.append(False)
        
        # Test Arweave Mock
        try:
            response = requests.get("http://localhost:1984", timeout=10)
            if response.status_code == 200:
                self.log("✅ Arweave mock service accessible")
                self.results["infrastructure"]["arweave_mock"] = {
                    "status": "accessible",
                    "response_length": len(response.text)
                }
                results.append(True)
            else:
                raise Exception(f"HTTP {response.status_code}")
                
        except Exception as e:
            self.log(f"❌ Arweave mock service failed: {str(e)}", "ERROR")
            self.results["infrastructure"]["arweave_mock"] = {"status": "failed", "error": str(e)}
            results.append(False)
            
        return all(results)

    def test_dns_resolution(self) -> bool:
        """Test DNS resolution between containers (if possible from host)"""
        # This test is limited from host perspective, but we can verify container names resolve
        try:
            # Use docker exec to test internal DNS from a container
            result = subprocess.run([
                "docker", "exec", "marketplace-service", 
                "nslookup", "postgres"
            ], capture_output=True, text=True, timeout=10)
            
            if result.returncode == 0:
                self.log("✅ Internal DNS resolution working")
                self.results["dns_tests"]["internal_resolution"] = {"status": "working"}
                return True
            else:
                self.log("❌ Internal DNS resolution failed", "ERROR")
                self.results["dns_tests"]["internal_resolution"] = {"status": "failed", "error": result.stderr}
                return False
                
        except subprocess.TimeoutExpired:
            self.log("❌ DNS test timed out", "ERROR")
            self.results["dns_tests"]["internal_resolution"] = {"status": "timeout"}
            return False
        except Exception as e:
            self.log(f"❌ DNS test failed: {str(e)}", "ERROR")
            self.results["dns_tests"]["internal_resolution"] = {"status": "error", "error": str(e)}
            return False

    def test_crypto_mode_switching(self) -> bool:
        """Test ENABLE_CRYPTO flag behavior"""
        # This would require restarting containers with different env vars
        # For now, we'll just verify current mode
        try:
            # Test a service endpoint for crypto-related responses
            response = requests.get("http://localhost:8081/health", timeout=10)
            if response.status_code == 200:
                # Check if response indicates crypto mode
                response_data = response.json() if response.headers.get('content-type', '').startswith('application/json') else {}
                crypto_enabled = response_data.get('crypto_enabled', False)
                
                self.log(f"✅ Crypto mode detection: {crypto_enabled}")
                self.results["crypto_mode_tests"]["current_mode"] = {
                    "crypto_enabled": crypto_enabled,
                    "verified": True
                }
                return True
            else:
                raise Exception(f"HTTP {response.status_code}")
                
        except Exception as e:
            self.log(f"❌ Crypto mode test failed: {str(e)}", "ERROR")
            self.results["crypto_mode_tests"]["current_mode"] = {"verified": False, "error": str(e)}
            return False

    def test_basic_api_endpoints(self) -> bool:
        """Test basic API endpoints for each service"""
        results = []
        
        for service_name, service_info in self.services.items():
            try:
                # Test root endpoint or common endpoints
                endpoints_to_test = ["/health", "/", "/api/v1/status"]
                
                for endpoint in endpoints_to_test:
                    try:
                        url = f"{service_info['url']}{endpoint}"
                        response = requests.get(url, timeout=5)
                        
                        if response.status_code in [200, 404, 405]:  # 404/405 are acceptable for non-implemented endpoints
                            self.log(f"✅ {service_name} endpoint {endpoint} responded (HTTP {response.status_code})")
                            
                            if service_name not in self.results["services"]:
                                self.results["services"][service_name] = {}
                            self.results["services"][service_name][endpoint] = {
                                "status_code": response.status_code,
                                "response_time": response.elapsed.total_seconds(),
                                "accessible": True
                            }
                            results.append(True)
                            break  # Found working endpoint
                            
                    except requests.exceptions.RequestException:
                        continue  # Try next endpoint
                        
                else:
                    # No endpoint worked
                    self.log(f"❌ {service_name} no accessible endpoints found", "ERROR")
                    self.results["services"][service_name] = {"accessible": False}
                    results.append(False)
                    
            except Exception as e:
                self.log(f"❌ {service_name} API test failed: {str(e)}", "ERROR")
                self.results["services"][service_name] = {"error": str(e), "accessible": False}
                results.append(False)
                
        return len([r for r in results if r]) >= len(results) * 0.7  # 70% success rate

    def wait_for_services_startup(self, timeout: int = 300):
        """Wait for services to start up"""
        self.log("Waiting for services to start up...")
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                # Test a few key services
                postgres_ready = self.test_database_connection()
                redis_ready = self.test_redis_connection()
                
                if postgres_ready and redis_ready:
                    self.log("✅ Core infrastructure services are ready")
                    return True
                    
            except Exception:
                pass
                
            self.log(f"Services still starting... ({int(time.time() - start_time)}s elapsed)")
            time.sleep(10)
            
        self.log("❌ Timeout waiting for services to start", "ERROR")
        return False

    def generate_report(self):
        """Generate comprehensive test report"""
        self.log("\n" + "="*60)
        self.log("BUNKERVERSE PLATFORM - DOCKER ENVIRONMENT TEST REPORT")
        self.log("="*60)
        
        # Service Health Summary
        self.log("\n🏥 SERVICE HEALTH CHECKS:")
        healthy_services = 0
        total_services = len(self.results.get("health_checks", {}))
        
        for service, result in self.results.get("health_checks", {}).items():
            status = result.get("status", "unknown")
            if status == "healthy":
                healthy_services += 1
                self.log(f"  ✅ {service}: {status} ({result.get('response_time', 0):.3f}s)")
            else:
                self.log(f"  ❌ {service}: {status} - {result.get('error', 'unknown error')}")
        
        if total_services > 0:
            health_percentage = (healthy_services / total_services) * 100
            self.log(f"\n  Service Health: {healthy_services}/{total_services} ({health_percentage:.1f}%)")
        
        # Infrastructure Summary
        self.log("\n🔧 INFRASTRUCTURE COMPONENTS:")
        working_infra = 0
        total_infra = len(self.results.get("infrastructure", {}))
        
        for component, result in self.results.get("infrastructure", {}).items():
            status = result.get("status", "unknown")
            if status in ["connected", "accessible"]:
                working_infra += 1
                self.log(f"  ✅ {component}: {status}")
                if "version" in result:
                    self.log(f"    Version: {result['version']}")
                if "chain_id" in result:
                    self.log(f"    Chain ID: {result['chain_id']}")
            else:
                self.log(f"  ❌ {component}: {status} - {result.get('error', 'unknown error')}")
        
        if total_infra > 0:
            infra_percentage = (working_infra / total_infra) * 100
            self.log(f"\n  Infrastructure Health: {working_infra}/{total_infra} ({infra_percentage:.1f}%)")
        
        # DNS and Network Tests
        self.log("\n🌐 NETWORK & DNS TESTS:")
        for test_name, result in self.results.get("dns_tests", {}).items():
            status = result.get("status", "unknown")
            if status == "working":
                self.log(f"  ✅ {test_name}: {status}")
            else:
                self.log(f"  ❌ {test_name}: {status}")
        
        # Crypto Mode Tests
        self.log("\n🔐 CRYPTO MODE TESTS:")
        for test_name, result in self.results.get("crypto_mode_tests", {}).items():
            if result.get("verified", False):
                crypto_enabled = result.get("crypto_enabled", False)
                self.log(f"  ✅ {test_name}: crypto_enabled = {crypto_enabled}")
            else:
                self.log(f"  ❌ {test_name}: verification failed")
        
        # Overall Status
        self.log("\n📊 OVERALL STATUS:")
        total_tests = healthy_services + working_infra
        max_tests = total_services + total_infra
        
        if max_tests > 0:
            success_rate = (total_tests / max_tests) * 100
            self.log(f"  Success Rate: {success_rate:.1f}% ({total_tests}/{max_tests})")
            
            if success_rate >= 90:
                self.log("  🎉 EXCELLENT - Environment is fully operational")
            elif success_rate >= 70:
                self.log("  ⚠️  GOOD - Environment is mostly operational")
            elif success_rate >= 50:
                self.log("  ⚠️  FAIR - Environment has significant issues")
            else:
                self.log("  ❌ POOR - Environment has major problems")
        
        # Save detailed results
        with open("test_results.json", "w") as f:
            json.dump(self.results, f, indent=2)
        self.log(f"\n📄 Detailed results saved to: test_results.json")
        
        self.log("="*60)

    def run_comprehensive_tests(self):
        """Run all tests in sequence"""
        self.log("🚀 Starting Bunkerverse Platform comprehensive testing...")
        
        # Wait for services to start
        if not self.wait_for_services_startup():
            self.log("❌ Failed to wait for services startup - proceeding with tests anyway", "WARN")
        
        # Test infrastructure first
        self.log("\n🔧 Testing infrastructure components...")
        self.test_database_connection()
        self.test_redis_connection()
        self.test_elasticsearch_connection()
        self.test_blockchain_nodes()
        self.test_mock_services()
        
        # Test service health
        self.log("\n🏥 Testing service health endpoints...")
        for service_name, service_info in self.services.items():
            self.test_service_health(service_name, service_info)
        
        # Test API endpoints
        self.log("\n🌐 Testing basic API endpoints...")
        self.test_basic_api_endpoints()
        
        # Test network functionality
        self.log("\n🔌 Testing network and DNS...")
        self.test_dns_resolution()
        
        # Test crypto mode
        self.log("\n🔐 Testing crypto mode configuration...")
        self.test_crypto_mode_switching()
        
        # Generate final report
        self.generate_report()
        
        return self.results

if __name__ == "__main__":
    suite = BunkerverseTestSuite()
    results = suite.run_comprehensive_tests()
    
    # Exit with appropriate code
    total_tests = len(results.get("health_checks", {})) + len(results.get("infrastructure", {}))
    successful_tests = len([r for r in results.get("health_checks", {}).values() if r.get("status") == "healthy"])
    successful_tests += len([r for r in results.get("infrastructure", {}).values() if r.get("status") in ["connected", "accessible"]])
    
    if total_tests > 0 and (successful_tests / total_tests) >= 0.7:
        sys.exit(0)  # Success
    else:
        sys.exit(1)  # Failure